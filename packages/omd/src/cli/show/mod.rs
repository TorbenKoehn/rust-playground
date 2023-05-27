use std::{collections::HashMap, io::Write, path::PathBuf, vec};

use bollard::{
  container::Config,
  exec::{CreateExecOptions, StartExecResults},
  image::{BuildImageOptions, BuilderVersion},
  models::BuildInfo,
  service::HostConfig,
  Docker,
};

#[cfg(feature = "buildkit")]
use bollard::models::BuildInfoAux;
#[cfg(feature = "buildkit")]
use futures_util::stream::StreamExt;

use crate::error::Error;
pub async fn cli_show(files: Vec<PathBuf>) -> Result<(), Error> {
  let docker = Docker::connect_with_socket_defaults()?;

  let dockerfile = String::from("FROM mysql:5.7");
  let mut header = tar::Header::new_gnu();
  header.set_path("Dockerfile")?;
  header.set_size(dockerfile.len() as u64);
  header.set_mode(0o755);
  header.set_cksum();
  let mut tar = tar::Builder::new(Vec::new());
  tar.append(&header, dockerfile.as_bytes())?;

  let uncompressed = tar.into_inner()?;
  let mut c = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
  c.write_all(&uncompressed)?;
  let compressed = c.finish()?;

  println!("Building MySQL Container");
  let image_id = "omd-rdbms-mysql";
  let build_options = BuildImageOptions {
    t: image_id,
    dockerfile: "Dockerfile",
    version: BuilderVersion::BuilderBuildKit,
    pull: true,
    #[cfg(feature = "buildkit")]
    session: Some(String::from(image_id)),
    ..Default::default()
  };
  let mut image_build_stream = docker.build_image(build_options, None, Some(compressed.into()));

  #[cfg(feature = "buildkit")]
  while let Some(Ok(BuildInfo {
    aux: Some(BuildInfoAux::BuildKit(_inner)),
    ..
  })) = image_build_stream.next().await
  {
    // println!("Response: {:?}", inner);
  }

  let mut binds = vec![];
  for file in files {
    println!("Attaching f {}", file.to_str().unwrap().to_owned());
    let full_path = file.canonicalize()?.to_str().unwrap().to_owned();
    println!("Attaching dump {}", full_path);
    binds.push(format!(
      "{}:/dumps/{}.sql",
      full_path,
      sha256::digest(full_path.as_str())
    ))
  }

  println!("Creating MySQL Container");
  let container_options = Config {
    image: Some(image_id),
    host_config: Some(HostConfig {
      binds: Some(binds.clone()),
      ..Default::default()
    }),
    ..Default::default()
  };
  let container_create_response = docker
    .create_container::<&str, &str>(None, container_options)
    .await?;
  println!("Container created: {:?}", container_create_response);

  println!("Starting MySQL Container");
  docker
    .start_container::<String>(&container_create_response.id, None)
    .await?;
  println!("MySQL Container started");

  println!("Exceuting commands");
  let exec_options = CreateExecOptions {
    attach_stdout: Some(true),
    attach_stderr: Some(true),
    cmd: Some(vec!["env"]),
    ..Default::default()
  };
  let exec_results = docker
    .create_exec(&container_create_response.id, exec_options)
    .await?;
  let start_exec_results = docker.start_exec(&exec_results.id, None).await?;
  match start_exec_results {
    StartExecResults::Attached { mut output, .. } => {
      while let Some(Ok(msg)) = output.next().await {
        print!("{}", msg);
      }
    }
    _ => unreachable!(),
  }
  println!("Commands executed");

  println!("Stopping MySQL Container");
  docker
    .stop_container(&container_create_response.id, None)
    .await?;

  println!("Removing MySQL Container");
  docker
    .remove_container(&container_create_response.id, None)
    .await?;

  Ok(())
}

use std::io::Write;

use bollard::{
  container::Config,
  exec::{CreateExecOptions, StartExecResults},
  image::{BuildImageOptions, BuilderVersion},
  models::BuildInfo,
  service::{ContainerCreateResponse, HostConfig},
  Docker,
};

use bollard::models::BuildInfoAux;
use futures_util::stream::StreamExt;

use crate::error::Error;

pub struct Dockerfile<'a> {
  contents: &'a str,
  compressed_contents: Vec<u8>,
}

impl<'a> Dockerfile<'a> {
  pub fn build(contents: &str) -> Result<Dockerfile, Error> {
    let dockerfile = String::from(contents);
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

    Ok(Dockerfile {
      contents,
      compressed_contents: c.finish()?,
    })
  }

  fn contents(&self) -> &str {
    self.contents
  }

  fn compressed_contents(&self) -> &Vec<u8> {
    &self.compressed_contents
  }
}

pub struct Image<'a> {
  docker: &'a Docker,
  dockerfile: Dockerfile<'a>,
  tag: &'a str,
}

impl<'a> Image<'a> {
  pub async fn build(
    docker: &'a Docker,
    dockerfile: Dockerfile<'a>,
    tag: &'a str,
  ) -> Result<Image<'a>, Error> {
    let build_options = BuildImageOptions {
      t: tag,
      dockerfile: "Dockerfile",
      version: BuilderVersion::BuilderBuildKit,
      pull: true,
      session: Some(String::from(tag)),
      ..Default::default()
    };

    let mut image_build_stream = docker.build_image(
      build_options,
      None,
      Some(dockerfile.compressed_contents().to_owned().into()),
    );

    while let Some(Ok(BuildInfo {
      aux: Some(BuildInfoAux::BuildKit(_inner)),
      ..
    })) = image_build_stream.next().await
    {
      // println!("Response: {:?}", inner);
    }

    Ok(Self {
      docker,
      dockerfile,
      tag,
    })
  }

  pub fn tag(&self) -> &str {
    self.tag
  }
}

pub struct ContainerOptions<'a> {
  pub image: Image<'a>,
  pub binds: Vec<String>,
}

pub struct Container<'a> {
  docker: &'a Docker,
  create_response: ContainerCreateResponse,
}

impl<'a> Container<'a> {
  pub async fn create(
    docker: &'a Docker,
    options: ContainerOptions<'a>,
  ) -> Result<Container<'a>, Error> {
    let container_config = Config {
      image: Some(options.image.tag()),
      host_config: Some(HostConfig {
        binds: Some(options.binds),
        ..Default::default()
      }),
      ..Default::default()
    };

    let response = docker
      .create_container::<&str, &str>(None, container_config)
      .await?;
    Ok(Self {
      docker,
      create_response: response,
    })
  }

  pub async fn start(&mut self) -> Result<(), Error> {
    self
      .docker
      .start_container::<String>(&self.create_response.id, None)
      .await?;
    Ok(())
  }

  pub async fn execute(&mut self, command: Vec<String>) -> Result<(), Error> {
    let exec_options = CreateExecOptions {
      attach_stdout: Some(true),
      attach_stderr: Some(true),
      cmd: Some(command),
      ..Default::default()
    };
    let exec_results = self
      .docker
      .create_exec(&self.create_response.id, exec_options)
      .await?;
    let start_exec_results = self.docker.start_exec(&exec_results.id, None).await?;
    match start_exec_results {
      StartExecResults::Attached { mut output, .. } => {
        while let Some(Ok(msg)) = output.next().await {
          print!("{}", msg);
        }
      }
      _ => unreachable!(),
    }

    Ok(())
  }

  pub async fn stop(&mut self) -> Result<(), Error> {
    self
      .docker
      .stop_container(&self.create_response.id, None)
      .await?;
    Ok(())
  }

  pub async fn remove(&mut self) -> Result<(), Error> {
    self
      .docker
      .remove_container(&self.create_response.id, None)
      .await?;
    Ok(())
  }
}

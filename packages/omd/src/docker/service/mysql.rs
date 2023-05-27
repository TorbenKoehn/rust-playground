use std::path::{Path, PathBuf};

use bollard::Docker;

use crate::{
  docker::container::{Container, ContainerOptions, Dockerfile, Image},
  error::Error,
};

pub struct MySqlServerOptions<'a> {
  pub image_tag: &'a str,
  pub mysql_version: &'a str,
  pub dump_files: &'a Vec<PathBuf>,
}

impl<'a> Default for MySqlServerOptions<'a> {
  fn default() -> Self {
    MySqlServerOptions {
      image_tag: "omd-rdbms-mysql",
      mysql_version: "5.7",
      dump_files: &vec![],
    }
  }
}

pub struct MySqlServer<'a> {
  container: Container<'a>,
}

impl<'a> MySqlServer<'a> {
  pub async fn new(
    docker: &'a Docker,
    options: MySqlServerOptions<'a>,
  ) -> Result<MySqlServer<'a>, Error> {
    let dockerfile = Dockerfile::build(&format!("FROM mysql:{}", options.mysql_version))?;
    let image = Image::build(docker, dockerfile, options.image_tag).await?;
    let binds = options
      .dump_files
      .iter()
      .map(|path| {
        // let path = path.canonicalize()?.to_str().unwrap().to_owned();
        // format!("{}:/docker-entrypoint-initdb.d/{}", path, path)
        "".to_owned()
      })
      .collect::<Vec<_>>();
    let container = Container::create(
      docker,
      ContainerOptions {
        image,
        binds: vec![],
      },
    )
    .await?;
    Ok(MySqlServer { container })
  }

  pub async fn start(&mut self) -> Result<(), Error> {
    self.container.start().await?;
    Ok(())
  }

  pub async fn stop(&mut self) -> Result<(), Error> {
    self.container.stop().await?;
    Ok(())
  }
}

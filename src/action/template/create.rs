use log;
use std::process::exit;

use crate::action;
use crate::cli::input;
use crate::config::Config;
use crate::out;

use clap::ArgMatches;

pub fn create(config: &mut Config, args: &ArgMatches) {
  let repository_name = args.value_of("repository");
  let template_name = args.value_of("template");

  // TODO create template in given directory
  // Only allow to create template directly in a repository of it has no remote

  // Get repository
  let repository = match action::get_repository(&config, repository_name) {
    Ok(repository) => repository,
    Err(error) => {
      log::error!("{}", error);
      eprintln!("{}", error);
      exit(1)
    }
  };

  // Get template name from user input
  let template_name = if template_name.is_none() {
    match input::text("template name", false) {
      Ok(value) => value,
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1);
        },
    }
  } else {
    String::from(template_name.unwrap())
  };

  // validate name
  let templates = repository.get_templates();
  if templates.contains(&template_name) {
    out::error::template_exists(&template_name);
    exit(1)
  }

  let template_path = match repository.create_template(&template_name) {
    Ok(value) => value,
    Err(error) => {
      log::error!("{}", error);
      println!("{}", error);
      exit(1)
    }
  };

  out::success::template_created(&template_path.to_str().unwrap());
}

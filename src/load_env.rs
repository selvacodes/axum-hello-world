use dotenv::dotenv;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use snafu::prelude::*;
use std::{
    fmt::Debug,
    future::{self, Future},
    path::PathBuf,
};

fn default_port() -> u16 {
    3000
}

#[derive(Deserialize)]
pub struct EnvConfig {
    #[serde(default = "default_port")]
    port: u16,
}

impl EnvConfig {
    pub fn get_port(&self) -> u16 {
        self.port
    }
}

#[derive(Debug, Snafu)]
pub enum EnvMishap {
    #[snafu(display("Error when loading env from file {:#?}", error))]
    FileToEnvError { error: dotenv::Error },
    #[snafu(display("error: {:#?}", error))]
    EnvLoadError { error: envy::Error },
}

// pub struct LoadEnvs {}
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct LoadEnv<P>(P);

pub trait SyncEnvLoad {
    type OutputConfig;
    fn load_to_env_from_file() -> Result<PathBuf, EnvMishap>;
    fn load_env() -> Result<Self::OutputConfig, EnvMishap>;
}

pub trait ASyncEnvLoad {
    type OutputConfig;
    type LoadFromFile: Future<Output = Result<PathBuf, EnvMishap>>;
    type LoadToEnv: Future<Output = Result<Self::OutputConfig, EnvMishap>>;
    fn load_to_env_from_file() -> Self::LoadFromFile;
    fn load_env() -> Self::LoadToEnv;
}

impl<P> SyncEnvLoad for LoadEnv<P>
where
    P: DeserializeOwned,
{
    type OutputConfig = P;
    fn load_to_env_from_file() -> Result<PathBuf, EnvMishap> {
        let update_env_from_file = dotenv().map_err(|err| EnvMishap::FileToEnvError { error: err });
        // .map_err(|e| eprint!("error while loading from dotenv {}", e))

        update_env_from_file
    }

    fn load_env() -> Result<Self::OutputConfig, EnvMishap> {
        let _env = dotenv::dotenv().ok();
        envy::from_env::<Self::OutputConfig>().map_err(|err| EnvMishap::EnvLoadError { error: err })
    }
}

impl<P> ASyncEnvLoad for LoadEnv<P>
where
    P: DeserializeOwned,
{
    type OutputConfig = P;

    type LoadFromFile = future::Ready<Result<PathBuf, EnvMishap>>;
    type LoadToEnv = future::Ready<Result<Self::OutputConfig, EnvMishap>>;

    fn load_to_env_from_file() -> Self::LoadFromFile {
        let update_env_from_file = dotenv()
            // .map_err(|e| eprint!("error while loading from dotenv {}", e))
            .map_err(|err| EnvMishap::FileToEnvError { error: err });

        future::ready(update_env_from_file)
    }

    fn load_env() -> Self::LoadToEnv {
        let env = dotenv::dotenv()
            .map_err(|err| EnvMishap::FileToEnvError { error: err })
            .and_then(|_y| {
                envy::from_env::<Self::OutputConfig>()
                    .map_err(|err| EnvMishap::EnvLoadError { error: err })
            });

        // let x = envy::from_env::<EnvConfig>().map_err(|err| EnvMishap::EnvLoadError { error: err });
        future::ready(env)
    }
}

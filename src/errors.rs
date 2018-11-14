use docker;
use hyper;
use serde_json;
use std::io;

error_chain! {
  foreign_links {
    Connection(hyper::Error);
    Url(hyper::error::ParseError);
    Io(io::Error);
    Json(serde_json::error::Error);
    Docker(docker::DockerError);
  }

  errors {
    CouldNotConnect(host: String) {
      description("could not connect to Docker")
      display("could not connected to Docker at '{}'", &host)
    }

    NoCertPath {
      description("could not find DOCKER_CERT_PATH")
      display("could not find DOCKER_CERT_PATH")
    }

    ParseError(wanted: &'static str, input: String) {
      description("error parsing JSON from Docker")
      display("could not parse JSON for {} from Docker", wanted)
    }

    SslDisabled {
      description("Docker SSL support was disabled at compile time")
      display("Docker SSL support was disabled at compile time")
    }

    SslError(host: String) {
      description("could not connect to Docker using SSL")
      display("could not connect to Docker at '{}' using SSL", &host)
    }

    UnsupportedScheme(host: String) {
      description("unsupported Docker URL scheme")
      display("do not know how to connect to Docker at '{}'", &host)
    }

    Unknown(message: String) {
      description("unknown error")
      display("unknown error: {}", &message)
    }
  }
}

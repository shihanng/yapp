provider "github" {
  owner = "shihanng"
}

module "github" {
  source     = "github.com/shihanng/tf-github-repo?ref=v0.1.0"
  repository = "yapp"
}

terraform {
  required_version = "~> 1.11"

  required_providers {
    github = {
      source  = "integrations/github"
      version = "~> 6.0"
    }
  }

  cloud {
    organization = "shihan"

    workspaces {
      name = "yapp"
    }
  }
}

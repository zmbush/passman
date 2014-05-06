package keystore

import (
  "github.com/libgit2/git2go"
)

func Init(c *Config) error {
  _, err := git.InitRepository(c.Directory, false)

  if err != nil {
    return err
  }

  return nil
}

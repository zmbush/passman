package keystore

import (
  "github.com/libgit2/git2go"
)

func Init(directory string) error {
  _, err := git.InitRepository(directory, false)

  if err != nil {
    return err
  }

  return nil
}



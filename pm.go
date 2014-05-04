package main

import (
  "flag"
  "fmt"
  "os"

  "github.com/zmbush/passman/keystore"
)

func main() {
  flag.Parse()

  args := flag.Args()

  if len(args) <= 0 {
    fmt.Println("You must specify a command")
    os.Exit(1);
    return;
  }

  conf := keystore.LoadConfig()

  var err error
  switch args[0] {
    case "init": err = keystore.Init(conf.Get("directory"));
  }

  if err != nil {
    fmt.Println(err)
  }
}

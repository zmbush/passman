package main

import (
  "flag"
  "fmt"
  "os"

  "github.com/zmbush/passman/keystore"
)

func main() {
  conf := keystore.LoadConfig()

  flag.PrintDefaults()

  args := flag.Args()

  fmt.Println(args)

  if len(args) <= 0 {
    fmt.Println("You must specify a command")
    os.Exit(1);
    return;
  }

  var err error
  switch args[0] {
    case "init": err = keystore.Init(conf)
    default: fmt.Println("Unknown Command:", args[0])
  }

  if err != nil {
    fmt.Println(err)
  }
}

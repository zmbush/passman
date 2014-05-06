package keystore

import (
  "github.com/rakyll/globalconf"
  "os/user"
  "flag"
  "fmt"
)

type Config struct {
  Directory string
  Config* globalconf.GlobalConf
}

type flagValue struct {
  str string
}

func (f *flagValue) String() string {
  return f.str
}

func (f *flagValue) Set(value string) error {
  f.str = value
  return nil
}

func setFlag(name, value string) *flag.Flag {
  return &flag.Flag{Name: name, Value: &flagValue{str: value}}
}

func LoadConfig() (* Config) {
  var path string

  c := new(Config)

  u, err := user.Current()

  if err !=  nil {
    fmt.Println("Could not find current user", err)
    path = "/tmp/passman"
  }

  path = u.HomeDir + "/.passman"

  flag.StringVar(&c.Directory, "directory", path, "The output directory")

  flag.Parse()

  c.Config, _ = globalconf.New("passman")

  c.Config.ParseAll()

  return c
}

func (c* Config) Set(key, value string) {
  c.Config.Set("", setFlag(key, value))
}

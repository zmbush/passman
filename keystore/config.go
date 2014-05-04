package keystore

type Config struct {
  props map[string]string
}

 func LoadConfig() (* Config) {
  c := new(Config)
  c.props = make(map[string]string)
  c.props["directory"] = "/Users/zmbush/.passman"
  return c
}

func (c* Config) Get(name string) string {
  return c.props[name]
}

func (c* Config) Put(name, value string) {
  c.props[name] = value
}



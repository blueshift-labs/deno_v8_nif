# Duxtape

**TODO: Add description**

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `duxtape` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:duxtape, "~> 0.1.0"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at [https://hexdocs.pm/duxtape](https://hexdocs.pm/duxtape).

### How to run
* `iex -S mix`
* `{:ok, ref} = Duxtape.Native.compile("(function (a, b) { return a + b; })")`
* `Duxtape.Native.eval(ref, "1,2")`
* Cant eval again as the channel gets destroyed when it goes out of scope :(

### Production
* To run with erlang 22 use `RUSTLER_NIF_VERSION=2.14`
* To do a production-level compile and run it with shell `MIX_ENV=prod iex -S mix`

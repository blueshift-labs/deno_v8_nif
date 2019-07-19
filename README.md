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

### Eval Pointless Args
Deno is stupid and you can't get results out of the execution
You can call it with `Duxtape.Native.eval("var a = 5; a;")`

### Benchmarking
* `a =  fn -> :timer.tc(fn -> Duxtape.Native.eval("var b = []; for (var i = 0; i <= 10000; i++) { b.push(i); } b;") end) end`
* `res = Enum.map(0..100_000, fn _ -> Task.async(a) end) |> Enum.map(&Task.await/1)`
* `Enum.filter(res, fn {num, _} -> num >= 100_000 end) |> Enum.count(fn {num, _} -> num end)` count of things over 100_000 microseconds
* `Enum.max_by(res, fn {num, _} -> num end)` slowest running
* `(Enum.map(res, fn {time, _} -> time end) |> Enum.sum()) / 100_000` microsecond average

Ruby is about 1_000 microseconds and rust is ~580 with peak of 750 for running 1 script and not considering parallelism (basically helloworld so it should scale worse when factoring in script complexity)

### Production
* To run with erlang 22 use `RUSTLER_NIF_VERSION=2.14`
* To do a production-level compile and run it with shell `MIX_ENV=prod iex -S mix`

defmodule DuxtapeTest do
  use ExUnit.Case
  doctest Duxtape

  test "greets the world" do
    assert Duxtape.hello() == :world
  end
end

defmodule GuaranaTest do
  use ExUnit.Case
  doctest Guarana

  test "greets the world" do
    assert Guarana.hello() == :world
  end
end

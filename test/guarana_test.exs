defmodule GuaranaTest do
  use ExUnit.Case
  doctest Guarana

  describe "derive_keypair" do
    test "derives keypair from private key" do
      {public_key, private_key} = Guarana.generate_keypair()

      {public_key, private_key, chain_code} =
        Guarana.derive_keypair(private_key, "hello", [1, 20, 15])
    end
  end
end

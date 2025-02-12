defmodule GuaranaTest do
  use ExUnit.Case
  doctest Guarana

  describe "master_key_from_seed/2" do
    test "generates a master key from seed" do
      seed = :crypto.strong_rand_bytes(32)

      {:ok,
       %Guarana.Key{
         depth: 0,
         index: 0,
         chain_code: _,
         signing_key: <<_private_key::binary-32, _public_key::binary-32>> = signing_key
       }} = Guarana.master_key_from_seed(seed)

      verify_signing(signing_key)
    end

    test "generates a master key from seed with custom hmac key" do
      seed = :crypto.strong_rand_bytes(32)

      {:ok,
       %Guarana.Key{
         depth: 0,
         index: 0,
         chain_code: _,
         signing_key: <<_private_key::binary-32, _public_key::binary-32>> = signing_key
       }} = Guarana.master_key_from_seed(seed, "hello")

      verify_signing(signing_key)
    end
  end

  describe "derive_child_key/2" do
    test "derives child key" do
      seed = :crypto.strong_rand_bytes(32)
      {:ok, master_key} = Guarana.master_key_from_seed(seed)

      base_path = "m/0'/1'/2'/2'/"

      Enum.each(1..100, fn index ->
        path = "#{base_path}#{index}'"

        {:ok,
         %Guarana.Key{
           depth: 5,
           index: key_index,
           chain_code: _,
           signing_key: <<_private_key::binary-32, _public_key::binary-32>> = signing_key
         }} = Guarana.derive_child_key(master_key, path)

        assert index == key_index - 2_147_483_648

        verify_signing(signing_key)
      end)
    end
  end

  defp verify_signing(<<_private_key::binary-32, public_key::binary>> = signing_key) do
    message = :crypto.strong_rand_bytes(64)
    assert {:ok, signature} = Cafezinho.sign(message, signing_key)

    assert :ok = Cafezinho.verify(signature, message, public_key)
  end
end

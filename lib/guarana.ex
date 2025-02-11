defmodule Guarana do
  alias Guarana.Impl

  def derive_keypair(key, salt, path) do
    <<secret_key::binary-32, chain_code::binary-32>> = :crypto.mac(:hmac, :sha512, salt, key)

    Impl.derive_key(secret_key, chain_code, path)
  end

  defdelegate keypair_from_seed(seed), to: Cafezinho

  defdelegate generate_keypair(), to: Cafezinho, as: :generate
end

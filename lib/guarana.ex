defmodule Guarana do
  alias Guarana.DerivationImpl
  alias Guarana.Key

  def master_key_from_seed(seed, hmac_key \\ "Gua") do
    case DerivationImpl.master_key_from_seed(seed, hmac_key) do
      {:ok, data} ->
        {:ok, create_key(data)}

      error ->
        error
    end
  end

  def derive_child_key(key, path) do
    case DerivationImpl.derive_child_key(
           key.depth,
           key.index,
           key.signing_key,
           key.chain_code,
           path
         ) do
      {:ok, data} ->
        {:ok, create_key(data)}

      error ->
        error
    end
  end

  def create_key({depth, child_index, signing_key, chain_code}) do
    %Key{depth: depth, index: child_index, chain_code: chain_code, signing_key: signing_key}
  end
end

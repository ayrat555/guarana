defmodule Guarana.DerivationImpl do
  @moduledoc false

  version = Mix.Project.config()[:version]

  use RustlerPrecompiled,
    otp_app: :guarana,
    crate: :guarana,
    base_url: "https://github.com/ayrat555/guarana/releases/download/v#{version}",
    force_build: true,
    targets: Enum.uniq(["x86_64-unknown-freebsd" | RustlerPrecompiled.Config.default_targets()]),
    nif_versions: ["2.15", "2.16"],
    version: version

  def master_key_from_seed(_seed, _hmac_key), do: :erlang.nif_error(:nif_not_loaded)

  def derive_child_key(_depth, _child_index, _signing_key, _chain_code, _path),
    do: :erlang.nif_error(:nif_not_loaded)
end

defmodule Guarana.Impl do
  @moduledoc false

  version = Mix.Project.config()[:version]

  use RustlerPrecompiled,
    otp_app: :guarana,
    crate: :guarana,
    base_url: "https://github.com/ayrat555/guaranag/releases/download/v#{version}",
    force_build: true,
    targets: Enum.uniq(["x86_64-unknown-freebsd" | RustlerPrecompiled.Config.default_targets()]),
    nif_versions: ["2.15", "2.16"],
    version: version

  def derive_key(_secret_key, _chain_code, _raw_path), do: :erlang.nif_error(:nif_not_loaded)
end

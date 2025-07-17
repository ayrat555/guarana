defmodule Guarana.MixProject do
  use Mix.Project

  @source_url "https://github.com/ayrat555/guarana"
  @version "0.2.0"

  def project do
    [
      app: :guarana,
      name: "Guarana",
      version: @version,
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      docs: docs(),
      package: package(),
      description: description()
    ]
  end

  defp description do
    """
    BIP32 key derivation for ed25519 keys
    """
  end

  defp package do
    [
      name: :guarana,
      maintainers: ["Ayrat Badykov"],
      licenses: ["MIT"],
      links: %{
        "Changelog" => "#{@source_url}/blob/master/CHANGELOG.md",
        "GitHub" => @source_url
      },
      files: [
        "mix.exs",
        "native/guarana/.cargo/config.toml",
        "native/guarana/src",
        "native/guarana/Cargo.toml",
        "native/guarana/Cargo.lock",
        "lib",
        "LICENSE",
        "README.md",
        "CHANGELOG.md",
        "checksum-*.exs"
      ]
    ]
  end

  defp docs do
    [
      main: "readme",
      source_url: @source_url,
      extras: [
        "CHANGELOG.md",
        "README.md"
      ]
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:ex_doc, ">= 0.0.0", only: :dev, runtime: false},
      {:credo, "~> 1.7", only: [:dev, :test], runtime: false},
      {:dialyxir, "~> 1.4", only: [:dev, :test], runtime: false},
      {:rustler, ">= 0.0.0", optional: true},
      {:cafezinho, "~> 0.4.4"},
      {:rustler_precompiled, "~> 0.8"}

      # {:dep_from_hexpm, "~> 0.3.0"},
      # {:dep_from_git, git: "https://github.com/elixir-lang/my_dep.git", tag: "0.1.0"}
    ]
  end
end

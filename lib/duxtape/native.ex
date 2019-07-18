defmodule Duxtape.Native do
  use Rustler, otp_app: :duxtape, crate: :duxtape

  def eval(_any), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end

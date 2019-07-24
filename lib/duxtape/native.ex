defmodule Duxtape.Native do
  use Rustler, otp_app: :duxtape, crate: :duxtape

  def compile(_any), do: error()
  def eval(_any, _str), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end

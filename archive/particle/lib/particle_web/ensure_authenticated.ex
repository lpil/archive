defmodule ParticleWeb.EnsureAuthenticated do
  @moduledoc """
  A Plug middleware that checks for a logged in user, and assigns the
  user data to the Plug conn. If the user is not logged in it redirects
  the user elsewhere.
  """

  alias Particle.User
  alias ParticleWeb.Router
  import ParticleWeb.Gettext, only: [gettext: 1]

  def init(opts) do
    opts
  end

  def call(conn, _opts) do
    case Plug.Conn.get_session(conn, :uid) |> User.fetch() do
      {:ok, user} ->
        conn
        |> Plug.Conn.assign(:user, user)

      :not_found ->
        conn
        |> Phoenix.Controller.put_flash(:error, gettext("Please login to continue"))
        |> Phoenix.Controller.redirect(to: Router.Helpers.page_path(conn, :index))
        |> Plug.Conn.halt()
    end
  end
end

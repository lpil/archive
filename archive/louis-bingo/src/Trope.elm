module Trope exposing (Trope(..), toString)


type Trope
    = Javascript
    | Elixir
    | TankTop
    | Hummus
    | Giphy


toString : Trope -> String
toString trope =
    case trope of
        Javascript ->
            "'OMGWTF Javascript!' :("

        Elixir ->
            "'This would be way better with Elixir!'"

        TankTop ->
            "Wearing a tank top"

        Hummus ->
            "'Dear god I love hummus'"

        Giphy ->
            "Reacts with Giphy on Slack"

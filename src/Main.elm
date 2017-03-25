module Main exposing (..)

import Html
import Types exposing (..)
import State
import View
import Backend
import CreateUser
import Backend.CreateEvent
import Backend.FetchEvents
import EventForm.State


main : Program Flags Model Msg
main =
    Html.programWithFlags
        { init = init
        , view = View.root
        , update = State.update
        , subscriptions = subscriptions
        }


init : Flags -> ( Model, Cmd Msg )
init flags =
    let
        send =
            Backend.cmd flags.idToken flags.endpoint

        createUser =
            send (CreateUser.query flags.idToken)
    in
        { newEvent = EventForm.State.init
        , createEvent = Backend.CreateEvent.query >> send
        , fetchEvents = send Backend.FetchEvents.query
        , events = Nothing
        }
            ! [ createUser
              ]


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none

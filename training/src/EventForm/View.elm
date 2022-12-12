module EventForm.View exposing (form)

import Date exposing (Date)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onInput, onSubmit)
import EventForm.Types exposing (..)
import EventForm.Date exposing (dateYyMmDd)
import Array exposing (Array)


form : Model -> Maybe Date -> Html Msg
form event maybeDate =
    let
        field =
            eventInput maybeDate
    in
        Html.form [ onSubmit Submit ]
            [ div [ class "hexagon" ] []
            , field event.name Name
            , field event.url Url
            , field event.dateEnd DateEnd
            , field event.dateEnd DateEnd
            , interestedPeople event.interestedPeople
            , div [] [ button [] [ text "Submit" ] ]
            ]


eventInput : Maybe Date -> String -> EventField -> Html Msg
eventInput currentDate currentValue field =
    let
        attrs =
            [ type_ <| inputType field
            , value currentValue
            , required True
            , onInput (Input field)
            ]
                ++ extraAttrs currentDate field
    in
        div [ class "event-form__field" ]
            [ label [] [ text <| labelText field ]
            , input attrs []
            ]


interestedPeople : Array String -> Html Msg
interestedPeople people =
    let
        nameInput i person =
            div []
                [ input
                    ([ type_ <| inputType (InterestedPeople i)
                     , value person
                     , onInput (Input (InterestedPeople i))
                     ]
                        ++ extraAttrs Nothing (InterestedPeople i)
                    )
                    []
                ]

        inputs =
            people
                |> Array.toIndexedList
                |> List.map (\( i, person ) -> nameInput i person)
    in
        div [ class "event-form__field" ] <|
            [ label [] [ text (labelText (InterestedPeople 0)) ] ]
                ++ inputs
                ++ [ (nameInput (Array.length people) "") ]


extraAttrs : Maybe Date -> EventField -> List (Attribute Msg)
extraAttrs maybeDate field =
    case ( field, maybeDate ) of
        ( DateStart, Just date ) ->
            [ Html.Attributes.min (dateYyMmDd date) ]

        ( DateEnd, Just date ) ->
            [ Html.Attributes.min (dateYyMmDd date) ]

        ( _, _ ) ->
            []


inputType : EventField -> String
inputType field =
    case field of
        Name ->
            "text"

        Url ->
            "url"

        DateStart ->
            "date"

        DateEnd ->
            "date"

        InterestedPeople _ ->
            "text"


labelText : EventField -> String
labelText field =
    case field of
        Name ->
            "Name"

        Url ->
            "URL"

        DateStart ->
            "Start Date"

        DateEnd ->
            "End Date"

        InterestedPeople _ ->
            "Who's interested"

module Sudoku exposing (SudokuBoard, viewSudoku)

import Canvas exposing (..)
import Canvas.Settings exposing (..)
import Canvas.Settings.Advanced exposing (GlobalCompositeOperationMode(..), transform, translate)
import Canvas.Settings.Text exposing (..)
import Color
import Grid
import Html exposing (..)
import Html.Attributes exposing (style)


type alias SudokuBoard =
    { cells : List (List Int)
    , width : Int
    , height : Int
    }


tileWidth : Float
tileWidth =
    50


tileHeight : Float
tileHeight =
    50


clearCanvas : Float -> Float -> Renderable
clearCanvas width height =
    shapes [ fill Color.white ] [ rect ( 0, 0 ) width height ]


renderTile : ( Float, Float ) -> Int -> Renderable
renderTile ( x, y ) value =
    let
        textX =
            tileWidth / 2

        textY =
            tileHeight / 2

        renderedText =
            Canvas.text
                [ align Center ]
                ( textX, textY )
                (String.fromInt value)

        renderedTile =
            shapes
                [ fill Color.white, stroke Color.black ]
                [ rect ( 0, 0 ) tileWidth tileHeight ]
    in
    group [ transform [ translate x y ] ]
        [ renderedTile
        , renderedText
        ]


renderTiles : SudokuBoard -> Renderable
renderTiles { cells, width, height } =
    let
        tiles =
            Grid.fold2d { rows = height, cols = width }
                (\( x, y ) result ->
                    renderTile
                        ( toFloat x * tileWidth, toFloat y * tileHeight )
                        (y * 10 + x)
                        :: result
                )
                []
    in
    group []
        tiles


viewSudoku : SudokuBoard -> Html msg
viewSudoku board =
    let
        width =
            500

        height =
            500
    in
    Canvas.toHtml ( width, height )
        []
        [ clearCanvas width height
        , renderTiles board
        ]

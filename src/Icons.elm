module Icons exposing (checkCircle, chevronDown, chevronLeft, chevronRight, chevronUp, circle, github, info, mail, menu, telegram, x, initprism)

import Html exposing (Html)
import Color exposing (rgb255)
import TypedSvg exposing (line, path, polyline, svg, g)
import TypedSvg.Attributes exposing (class, cx, cy, d, points, r, viewBox, x1, x2, y1, y2, width, height, viewBox, transform, fill)
import TypedSvg.Core exposing (Svg)
import TypedSvg.Types exposing (ClipPath(..), Fill(..), Transform(..), Scale(..), Fill(..), px, pt)


svgIcon : String -> List (Svg msg) -> Html msg
svgIcon className =
    svg
        [ class [ "icon ", className ]
        , viewBox 0 0 24 24
        ]


checkCircle : Html msg
checkCircle =
    svgIcon "check-circle"
        [ path [ d "M22 11.08V12a10 10 0 1 1-5.93-9.14" ] []
        , polyline [ points [ ( 22, 4 ), ( 12, 14.01 ), ( 9, 11.01 ) ] ] []
        ]


chevronDown : Html msg
chevronDown =
    svgIcon "chevron-down"
        [ polyline [ points [ ( 6, 9 ), ( 12, 15 ), ( 18, 9 ) ] ] []
        ]


chevronUp : Html msg
chevronUp =
    svgIcon "chevron-up"
        [ polyline [ points [ ( 18, 15 ), ( 12, 9 ), ( 6, 15 ) ] ] []
        ]


chevronLeft : Html msg
chevronLeft =
    svgIcon "chevron-left"
        [ polyline [ points [ ( 15, 18 ), ( 9, 12 ), ( 15, 6 ) ] ] []
        ]


chevronRight : Html msg
chevronRight =
    svgIcon "chevron-right"
        [ polyline [ points [ ( 9, 18 ), ( 15, 12 ), ( 9, 6 ) ] ] []
        ]


circle : Html msg
circle =
    svgIcon "circle"
        [ TypedSvg.circle [ cx (px 12), cy (px 12), r (px 10) ] []
        ]


info : Html msg
info =
    svgIcon "info"
        [ TypedSvg.circle [ cx (px 12), cy (px 12), r (px 10) ] []
        , line [ x1 (px 12), y1 (px 16), x2 (px 12), y2 (px 12) ] []
        , line [ x1 (px 12), y1 (px 8), x2 (px 12), y2 (px 8) ] []
        ]


github : Html msg
github =
    svgIcon "github"
        [ path [ d "M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22" ] []
        ]


mail : Html msg
mail =
    svgIcon "mail"
        [ path [ d "M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z" ] []
        , polyline [ points [ ( 22, 6 ), ( 12, 13 ), ( 2, 6 ) ] ] []
        ]


menu : Html msg
menu =
    svgIcon "menu"
        [ line [ x1 (px 3), y1 (px 12), x2 (px 21), y2 (px 12) ] []
        , line [ x1 (px 3), y1 (px 6), x2 (px 21), y2 (px 6) ] []
        , line [ x1 (px 3), y1 (px 18), x2 (px 21), y2 (px 18) ] []
        ]


telegram : Html msg
telegram =
    svgIcon "telegram"
        [ path [ d "M23.932 3.769l-3.622 17.08c-.273 1.205-.986 1.505-1.999.937l-5.518-4.066-2.663 2.561c-.294.294-.541.541-1.109.541l.397-5.62L19.646 5.96c.444-.397-.097-.616-.692-.22L6.31 13.702.867 11.998c-1.184-.37-1.205-1.184.247-1.752l21.291-8.203c.985-.369 1.848.22 1.527 1.726z" ] []
        ]


x : Html msg
x =
    svgIcon "x"
        [ line [ x1 (px 18), y1 (px 6), x2 (px 6), y2 (px 18) ] []
        , line [ x1 (px 6), y1 (px 6), x2 (px 18), y2 (px 18) ] []
        ]

-- blog logo
initprism : Html msg
initprism =
    svg
        [ class [ "icon" ]
        , viewBox 0 0 310 310
        , width (pt 310)
        , height (pt 310)
        ]
        [ 
            g [ transform [ Translate 0 310 
                          , Scale 0.1 -0.1
                          ] 
              , fill <| Fill <| rgb255 179 193 217
              ]
              [
               path [ d "M1410 3089 c-249 -37 -368 -71 -545 -158 -153 -75 -274 -161 -396 -281 -287 -283 -439 -628 -466 -1060 -8 -128 27 -355 79 -515 126 -383 380 -686 742 -884 295 -161 675 -217 1020 -150 674 131 1187 706 1235 1385 21 307 -32 577 -164 832 -203 391 -567 681 -985 785 -181 45 -395 64 -520 46z m-415 -1266 c206 -252 217 -267 204 -288 -8 -13 -104 -133 -214 -268 l-200 -246 -189 -1 c-156 0 -187 2 -183 14 3 7 96 125 206 260 111 136 201 252 201 259 0 6 -41 62 -91 122 -251 306 -320 395 -314 405 4 6 72 10 184 10 l178 0 218 -267z m414 150 l93 -113 589 0 c571 0 589 -1 599 -19 11 -21 14 -179 4 -205 -5 -14 -58 -16 -495 -16 -269 0 -489 -3 -489 -7 0 -5 9 -19 20 -33 18 -24 18 -25 -1 -57 l-21 -33 486 0 c470 0 486 -1 496 -19 11 -21 14 -179 4 -205 -6 -14 -67 -16 -593 -16 -322 0 -591 -4 -596 -8 -6 -4 -50 -55 -99 -114 l-88 -108 -184 0 c-123 0 -184 4 -184 11 0 5 88 118 195 250 107 131 200 247 206 258 11 16 -15 51 -199 276 -116 142 -209 262 -206 267 3 5 87 7 187 6 l182 -3 94 -112z" ] []
              ]
        ]

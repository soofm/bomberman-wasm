(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "./node_modules/@victorlee/bomberman-wasm sync recursive":
/*!*****************************************************!*\
  !*** ./node_modules/@victorlee/bomberman-wasm sync ***!
  \*****************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("function webpackEmptyContext(req) {\n\tvar e = new Error(\"Cannot find module '\" + req + \"'\");\n\te.code = 'MODULE_NOT_FOUND';\n\tthrow e;\n}\nwebpackEmptyContext.keys = function() { return []; };\nwebpackEmptyContext.resolve = webpackEmptyContext;\nmodule.exports = webpackEmptyContext;\nwebpackEmptyContext.id = \"./node_modules/@victorlee/bomberman-wasm sync recursive\";\n\n//# sourceURL=webpack:///./node_modules/@victorlee/bomberman-wasm_sync?");

/***/ }),

/***/ "./src/InputController.ts":
/*!********************************!*\
  !*** ./src/InputController.ts ***!
  \********************************/
/*! exports provided: PlayerOneInputOptions, PlayerTwoInputOptions, InputController */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"PlayerOneInputOptions\", function() { return PlayerOneInputOptions; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"PlayerTwoInputOptions\", function() { return PlayerTwoInputOptions; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"InputController\", function() { return InputController; });\n/* harmony import */ var _victorlee_bomberman_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! @victorlee/bomberman-wasm */ \"./node_modules/@victorlee/bomberman-wasm/bomberman_wasm.js\");\n\r\nconst PlayerOneInputOptions = {\r\n    left: \"a\",\r\n    right: \"d\",\r\n    up: \"w\",\r\n    down: \"s\",\r\n    bomb: \"e\"\r\n};\r\nconst PlayerTwoInputOptions = {\r\n    left: \"ArrowLeft\",\r\n    right: \"ArrowRight\",\r\n    up: \"ArrowUp\",\r\n    down: \"ArrowDown\",\r\n    bomb: \"Enter\"\r\n};\r\nclass InputController {\r\n    constructor(sendInput) {\r\n        this.sendInput = sendInput;\r\n    }\r\n    registerInputs(canvas, inputOptions) {\r\n        if (this.kdEventListener) {\r\n            canvas.removeEventListener(\"keydown\", this.kdEventListener);\r\n        }\r\n        if (this.kuEventListener) {\r\n            canvas.removeEventListener(\"keyup\", this.kuEventListener);\r\n        }\r\n        const inputMapping = new Map();\r\n        inputOptions.forEach((inputOption, id) => {\r\n            inputMapping.set(inputOption.left, { id, key: _victorlee_bomberman_wasm__WEBPACK_IMPORTED_MODULE_0__[\"InputType\"].Left });\r\n            inputMapping.set(inputOption.right, { id, key: _victorlee_bomberman_wasm__WEBPACK_IMPORTED_MODULE_0__[\"InputType\"].Right });\r\n            inputMapping.set(inputOption.up, { id, key: _victorlee_bomberman_wasm__WEBPACK_IMPORTED_MODULE_0__[\"InputType\"].Up });\r\n            inputMapping.set(inputOption.down, { id, key: _victorlee_bomberman_wasm__WEBPACK_IMPORTED_MODULE_0__[\"InputType\"].Down });\r\n            inputMapping.set(inputOption.bomb, { id, key: _victorlee_bomberman_wasm__WEBPACK_IMPORTED_MODULE_0__[\"InputType\"].Bomb });\r\n        });\r\n        this.kdEventListener = (event) => {\r\n            const value = inputMapping.get(event.key);\r\n            if (typeof value !== \"undefined\") {\r\n                this.sendInput(value.id, value.key, true);\r\n            }\r\n            event.preventDefault();\r\n        };\r\n        canvas.addEventListener(\"keydown\", this.kdEventListener);\r\n        this.kuEventListener = (event) => {\r\n            const value = inputMapping.get(event.key);\r\n            if (typeof value !== \"undefined\") {\r\n                this.sendInput(value.id, value.key, false);\r\n            }\r\n            event.preventDefault();\r\n        };\r\n        canvas.addEventListener(\"keyup\", this.kuEventListener);\r\n    }\r\n}\r\n\n\n//# sourceURL=webpack:///./src/InputController.ts?");

/***/ }),

/***/ "./src/bindings.ts":
/*!*************************!*\
  !*** ./src/bindings.ts ***!
  \*************************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _victorlee_bomberman_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! @victorlee/bomberman-wasm */ \"./node_modules/@victorlee/bomberman-wasm/bomberman_wasm.js\");\n/* harmony import */ var _InputController__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./InputController */ \"./src/InputController.ts\");\n\r\n\r\nconst main = document.getElementById(\"main\");\r\nconst initMenu = document.getElementById(\"init-menu\");\r\nconst postMenu = document.getElementById(\"post-menu\");\r\nconst canvas = document.getElementById(\"canvas\");\r\nconst ctx = canvas.getContext(\"2d\");\r\nconst onePlayer = document.getElementById(\"one-player\");\r\nconst twoPlayer = document.getElementById(\"two-player\");\r\nlet animationId = null;\r\nlet gameEngine = _victorlee_bomberman_wasm__WEBPACK_IMPORTED_MODULE_0__[\"GameEngine\"].new();\r\nconst sendInput = (playerId, inputType, on) => {\r\n    gameEngine.send_input(playerId, inputType, on);\r\n};\r\nconst inputController = new _InputController__WEBPACK_IMPORTED_MODULE_1__[\"InputController\"](sendInput);\r\nconst renderLoop = () => {\r\n    gameEngine.draw(ctx);\r\n    const state = gameEngine.tick();\r\n    if (state != 0) {\r\n        stopGame(state);\r\n    }\r\n    else {\r\n        animationId = requestAnimationFrame(renderLoop);\r\n    }\r\n};\r\nconst startGame = () => {\r\n    if (initMenu.style.visibility !== \"hidden\") {\r\n        initMenu.style.visibility = \"hidden\";\r\n    }\r\n    else {\r\n        gameEngine.reset();\r\n    }\r\n    main.style.visibility = \"hidden\";\r\n    postMenu.style.visibility = \"hidden\";\r\n    canvas.focus();\r\n    renderLoop();\r\n};\r\nconst stopGame = (state) => {\r\n    main.style.visibility = \"visible\";\r\n    postMenu.style.visibility = \"visible\";\r\n    if (state > 0) {\r\n        postMenu.firstElementChild.textContent = `Player ${state} wins!`;\r\n    }\r\n    else {\r\n        postMenu.firstElementChild.textContent = \"TIE!\";\r\n    }\r\n    cancelAnimationFrame(animationId);\r\n    animationId = null;\r\n};\r\nonePlayer.addEventListener(\"click\", (event) => {\r\n    gameEngine.set_human_player(0);\r\n    inputController.registerInputs(canvas, [_InputController__WEBPACK_IMPORTED_MODULE_1__[\"PlayerOneInputOptions\"]]);\r\n    startGame();\r\n});\r\ntwoPlayer.addEventListener(\"click\", (event) => {\r\n    gameEngine.set_human_player(0);\r\n    gameEngine.set_human_player(1);\r\n    inputController.registerInputs(canvas, [_InputController__WEBPACK_IMPORTED_MODULE_1__[\"PlayerOneInputOptions\"], _InputController__WEBPACK_IMPORTED_MODULE_1__[\"PlayerTwoInputOptions\"]]);\r\n    startGame();\r\n});\r\n\n\n//# sourceURL=webpack:///./src/bindings.ts?");

/***/ })

}]);
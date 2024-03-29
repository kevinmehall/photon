import * as preact from "preact";

/*
https://ionic.io/ionicons

Copyright (c) 2015-present Ionic (http://ionic.io/)

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
*/

export const AddCircle = () =>
    <svg xmlns="http://www.w3.org/2000/svg" class="icon" viewBox="0 0 512 512">
        <path fill="currentColor" d="M256 48C141.31 48 48 141.31 48 256s93.31 208 208 208 208-93.31 208-208S370.69 48 256 48zm80 224h-64v64a16 16 0 01-32 0v-64h-64a16 16 0 010-32h64v-64a16 16 0 0132 0v64h64a16 16 0 010 32z" />
    </svg>

export const AddCircleOutline = () =>
    <svg xmlns="http://www.w3.org/2000/svg" class="icon" viewBox="0 0 512 512">
        <path d="M448 256c0-106-86-192-192-192S64 150 64 256s86 192 192 192 192-86 192-192z" fill="none" stroke="currentColor" stroke-miterlimit="10" stroke-width="32" />
        <path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" d="M256 176v160M336 256H176" />
    </svg>

export const ChevronDown = () =>
    <svg xmlns="http://www.w3.org/2000/svg" class="icon" viewBox="0 0 512 512">
        <path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="48" d="M112 184l144 144 144-144" />
    </svg>

export const ChevronUp = () =>
    <svg xmlns="http://www.w3.org/2000/svg" class="icon" viewBox="0 0 512 512">
        <path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="48" d="M112 328l144-144 144 144" />
    </svg>

export const CloseOutline = () =>
    <svg xmlns="http://www.w3.org/2000/svg" class="icon" viewBox="0 0 512 512">
        <path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" d="M368 368L144 144M368 144L144 368" />
    </svg>

export const CopyOutline = () =>
    <svg xmlns="http://www.w3.org/2000/svg" class="icon" viewBox="0 0 512 512">
        <rect x="128" y="128" width="336" height="336" rx="57" ry="57" fill="none" stroke="currentColor" stroke-linejoin="round" stroke-width="32" />
        <path d="M383.5 128l.5-24a56.16 56.16 0 00-56-56H112a64.19 64.19 0 00-64 64v216a56.16 56.16 0 0056 56h24" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" />
    </svg>

export const WarningOutline = () =>
    <svg xmlns="http://www.w3.org/2000/svg" class="icon" viewBox="0 0 512 512">
        <path d="M85.57 446.25h340.86a32 32 0 0028.17-47.17L284.18 82.58c-12.09-22.44-44.27-22.44-56.36 0L57.4 399.08a32 32 0 0028.17 47.17z" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" />
        <path d="M250.26 195.39l5.74 122 5.73-121.95a5.74 5.74 0 00-5.79-6h0a5.74 5.74 0 00-5.68 5.95z" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" />
        <path d="M256 397.25a20 20 0 1120-20 20 20 0 01-20 20z" fill="currentColor" />
    </svg>

export const RemoveCircle = () =>
    <svg xmlns="http://www.w3.org/2000/svg" class="icon" viewBox="0 0 512 512">
        <path fill="currentColor" d="M256 48C141.31 48 48 141.31 48 256s93.31 208 208 208 208-93.31 208-208S370.69 48 256 48zm80 224H176a16 16 0 010-32h160a16 16 0 010 32z" />
    </svg>

export const RemoveCircleOutline = () =>
    <svg xmlns="http://www.w3.org/2000/svg" class="icon" viewBox="0 0 512 512">
        <path d="M448 256c0-106-86-192-192-192S64 150 64 256s86 192 192 192 192-86 192-192z" fill="none" stroke="currentColor" stroke-miterlimit="10" stroke-width="32" />
        <path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" d="M336 256H176" />
    </svg>

export const TimeOutline = () =>
    <svg xmlns="http://www.w3.org/2000/svg" class="icon" viewBox="0 0 512 512">
        <path d="M256 64C150 64 64 150 64 256s86 192 192 192 192-86 192-192S362 64 256 64z" fill="none" stroke="currentColor" stroke-miterlimit="10" stroke-width="32" />
        <path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" d="M256 128v144h96" />
    </svg>

export const CubeOutline = () =>
    <svg xmlns="http://www.w3.org/2000/svg" class="icon" viewBox="0 0 512 512">
        <path d="M448 341.37V170.61A32 32 0 00432.11 143l-152-88.46a47.94 47.94 0 00-48.24 0L79.89 143A32 32 0 0064 170.61v170.76A32 32 0 0079.89 369l152 88.46a48 48 0 0048.24 0l152-88.46A32 32 0 00448 341.37z" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" />
        <path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" d="M69 153.99l187 110 187-110M256 463.99v-200" />
    </svg>

export const CodeOutline = () =>
    <svg xmlns="http://www.w3.org/2000/svg" class="ionicon" viewBox="0 0 512 512">
        <path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" d="M160 368L32 256l128-112M352 368l128-112-128-112" />
    </svg>

@_list:
	just --list --unsorted


run: build
	npm run preview

build:
	npm install
	npm run build
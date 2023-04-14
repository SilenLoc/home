@_list:
	just --list --unsorted


run:
	npm run build
	npm run preview

build:
	npm install
	npm run build
# How To Run

- Run `npm install` to install dependencies
- Run `npm run start` to start the server

# Pull Request

- Run `npm run pull-request` before submitting pull request

# `src` Directory Structure

- assets
  - contains all images, css files, font files, etc
- components
  -
- data
  - stores data assets such as JSON files that contain information used in our code
  - e.g. theme information, global constant variables
- features
  - the logic for specific feature
- hooks
  - contains global hooks
- layouts
  - place any layout components
- lib
  - facade for various different libraries
- pages
  - contains all the pages
  - page comprises multiple components
  - import from features to make use of logic
- services
  - contains all the codes interfacing with any external API
- utils
  - contains utilities e.g. formatter

[Ref](https://blog.webdevsimplified.com/2022-07/react-folder-structure/)

# 02 - Backend API

The goal of this project is to create an http+json API for a calculator service.

## Overview

This calculator is stateless, meaning that there is no data stored in a database or in memory.

## Requirements

The API should conform to the given OpenAPI spec found in this directory.

### Production Ready

In order to make this API more production ready, there's a few other requiements you'll need to consider

#### Input Validation

You should never trust input from the user. This means you'll need to be sure to validate and sanitize any inputs. Division by zero is a common cause
panics when it comes to applications, so you'll want to make sure you're handling it.

#### Error feedback
Additionally, it's a good idea to let the user know when they've made a mistake with input, so they can fix it if the mistake was innocent.

#### Logging

In order to be able to debug issues that occur, you're going to want to log out each request as it comes in, as well as any associated data such as the status code, ip address, and what the request path was.

#### CORS

You may want to add cors in if you intend to hit this directly from a website.

## Additional Tasks

- Add in rate limiter to prevent misuse of the API
- Add in token authentication to prevent anyone unauthorized from using the API
- Add in a database to keep track of all of the calculations that have taken place
- Add in support for floating point numbers as well.
- Create an associated http client that can work with the calculator API.
- Create a frontend that makes use of your API.
- Add in a middleware that adds a request ID to the http.Request object.
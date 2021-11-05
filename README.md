# Group 0

## Members

Cooper Miller (kcm3)

## Project Introduction

I hope to create a REST API Wrapper for OpenAI's GPT-3 API. If I have time, then I'd like to try to fine-tune the model and create something similar to [this](https://youtu.be/21zDl4sX3nE).
I've had access to GPT-3 for a while now and thought this would be a good opportunity to start creating with it. I also want to gain more experience designing REST APIs.

## System Overview

First thing I'll need to do is read through the GPT-3 documentation and choose a GPT-3 engine to make requests to.
Next, I'll have to create rust API bindings for the OpenAI HTTP API. I'll most likely use the wrap and/or hyper crates to accomplish this, but I will have to look into other web frameworks before I decide.
If I can successfully query the engine and have enough time, then I'll try to fine-tune the model. Since I am considering replicating this [project](<(https://youtu.be/21zDl4sX3nE)>), I might use the [MAL API](https://myanimelist.net/apiconfig/references/api/v2) to get titles of novels to fine-tune the model with.

## Possible Challenges

I've never designed a REST API and I don't have much experience making API requests, so I believe figuring out how to interact with the model may be the biggest challenge I face.

Also, I have used GPT-3's prompt playground, but I've never tried to fine-tune it. I'm not sure how difficult that'll be but I believe one challenge I may face is finding enough data to fine-tune a model.

## References

If I have time, I'd like to create something like [this](https://youtu.be/21zDl4sX3nE).

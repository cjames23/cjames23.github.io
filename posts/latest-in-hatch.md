---
title: Latest developments in hatch
author: C. James
date: 2026-06-28T13:10:00Z
tags: [python, packaging, hatch]
---

### What's New in Hatch? 
Hatch has launched several new versions since I started acting as a co-maintainer.
The first contribution I made was workspaces which is monorepo support in hatch. 
Fun fact, hatch itself is actually a monorepo with hatchling which gave a good opportunity to test out workspaces.
We have since launched support in hatch to include lockfiles as a native feature. But even bigger was the
redsign of static analysis commands. We announced the new command `hatch check`.
Along with this redesign this allowed us to launch `hatch check types` which uses
`pyrefly` as the default type checker. 

#### Why Pyrefly
I was thoroughly impressed with Pyrefly during testing for hatch to determine which type checker
made the most sense to use as the default. We were looking for a type checker that would not
require us to maintain a significant amount of configuration ourselves. Pyrefly offered this
with modes like `legacy` that tries to make it easier to migrate from `mypy`. 


### What is coming next with Hatch
* I have been actively working on adding `sources` support for hatch. This should close
a gap for hatch to allow more native support for using other indexes for dependencies like pytorch.
Without having to directly configure `uv`. 
* We have been evaluating ruff rules again to make sure that we continue
to provide the best defaults out of the box for linting and formatting. 

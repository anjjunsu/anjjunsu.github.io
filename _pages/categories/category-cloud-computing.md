---
title: "Cloud-computing"
layout: archive
permalink: categories/cloud-computing
author_profile: true
sidebar_main: true
---
<!-- if category name includes space, format below as site.categories['a b c'] -->

***

{% assign posts = site.categories.CloudComputing %}
{% for post in posts %} {% include archive-single2.html type=page.entries_layout %} {% endfor %}
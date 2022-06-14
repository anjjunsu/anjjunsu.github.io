---
title: "AWS"
layout: archive
permalink: categories/aws
author_profile: true
sidebar_main: true
---
<!-- if category name includes space, format below as site.categories['a b c'] -->

***

{% assign posts = site.categories.AWS %}
{% for post in posts %} {% include archive-single2.html type=page.entries_layout %} {% endfor %}
---
title: "BACK-END"
layout: archive
permalink: categories/back-end
author_profile: true
sidebar_main: true
---
<!-- if category name includes space, format below as site.categories['a b c'] -->

***

{% assign posts = site.categories.BackEnd %}
{% for post in posts %} {% include archive-single2.html type=page.entries_layout %} {% endfor %}
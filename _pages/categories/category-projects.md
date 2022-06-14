---
title: "Projects"
layout: archive
permalink: categories/projects
author_profile: true
sidebar_main: true
---
<!-- if category name includes space, format below as site.categories['a b c'] -->

***

{% assign posts = site.categories.Projects %}
{% for post in posts %} {% include archive-single2.html type=page.entries_layout %} {% endfor %}
---
layout: default
title: Blog
permalink: /blog/
---

# Blog

Thoughts, updates, and deep dives into Claude Code observability.

<ul class="post-list">
{% for post in site.posts %}
  <li class="post-list-item">
    <span class="tag tag-date">{{ post.date | date: "%b %-d, %Y" }}</span>
    <h3 class="post-list-title">
      <a href="{{ post.url | relative_url }}">{{ post.title }}</a>
    </h3>
    {% if post.tags %}
    <p class="post-list-meta">
      {% for tag in post.tags %}
        <a href="{{ '/tags/#' | append: tag | slugify | relative_url }}" class="tag tag-{{ tag | slugify }}">{{ tag }}</a>
      {% endfor %}
    </p>
    {% endif %}
    {% if post.excerpt %}
      <p class="post-list-excerpt">{{ post.excerpt | strip_html | truncate: 200 }}</p>
    {% endif %}
  </li>
{% endfor %}
</ul>

{% if site.posts.size == 0 %}
<p class="text-muted">No posts yet. Check back soon!</p>
{% endif %}

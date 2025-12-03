---
layout: default
title: Tags
permalink: /tags/
---

# Tags

Browse posts by topic.

{% assign tags = site.posts | map: "tags" | compact | flatten | uniq | sort %}

<div class="tag-cloud">
{% for tag in tags %}
  <a href="#{{ tag | slugify }}" class="tag tag-{{ tag | slugify }}">{{ tag }}</a>
{% endfor %}
</div>

---

{% for tag in tags %}
<h2 id="{{ tag | slugify }}">{{ tag | capitalize }}</h2>

<ul class="post-list">
{% for post in site.posts %}
  {% if post.tags contains tag %}
  <li class="post-list-item">
    <h3 class="post-list-title">
      <a href="{{ post.url | relative_url }}">{{ post.title }}</a>
    </h3>
    <p class="post-list-meta">
      <span>{{ post.date | date: "%B %-d, %Y" }}</span>
    </p>
  </li>
  {% endif %}
{% endfor %}
</ul>

{% endfor %}

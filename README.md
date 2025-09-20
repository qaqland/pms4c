# pms4c

很久很久以前，中文写 markdown 是不怎么能换行的，换行会导致渲染出来的 HTML
中多一个空白字符，于是我准备做个小工具作为预处理器去吃掉这个 SoftBreak。

基本搓好了 mdbook 预处理插件，测试的时候发现，原来火狐已经针对该问题做了修改

- <https://phabricator.services.mozilla.com/D231476>

到此为止了（发出索杰恩的声音）

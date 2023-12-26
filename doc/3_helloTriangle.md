东西被封装到了Engine中。

现有逻辑：

    - engine先new，包含渲染组件和wgpu state，顺便返回event_loop因为我不知道这玩意的move后面该怎么写

一个需要思考的问题：

    - render_pass的意义是什么

    渲染到不同的屏幕吧。

    gui和最后图像一起渲染；

    或者是把最后的图像渲染到gui中。

    多个renderpass是为了渲染阴影视角等。

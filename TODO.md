# InFantasyShell TODO (v1)

## Phase 1 — Runtime & Shell（基础运行环境）

- [ ] Bash-like Command Line（POSIX-like 子集）
   - [ ] Core
      - [ ] Prompt（带路径 + bit 信息）
         - [ ] DoD: 显示 player@world:/area$ [32B/64B]
      - [ ] Parse Input（基础解析器）
         - [ ] DoD: 能解析 command + args（如 cat a/b）
         - [ ] Execute Command（命令调度）
            - [ ] DoD: 命令正确路由执行
         - [ ] Print Output（标准输出）
            - [ ] DoD: 输出格式统一
         - [ ] Handle Errors（错误系统）
            - [ ] DoD: 统一错误（not found / permission / overflow）

   - [ ] Minimal Command Set
      - [ ] cd
         - [ ] DoD: 支持相对/绝对路径
      - [ ] ls
         - [ ] DoD: 列出目录内容
      - [ ] pwd
         - [ ] DoD: 显示当前路径
      - [ ] cat
         - [ ] DoD: 读取文件（受 bit 限制）
      - [ ] echo
         - [ ] DoD: 支持写入文件（>）
      - [ ] rm
         - [ ] DoD: 删除文件/目录
      - [ ] chmod
         - [ ] DoD: 修改权限

   - [ ] 延后实现（v2）
      - [ ] piping（|）
      - [ ] redirection（完整）
      - [ ] background jobs（&）
      - [ ] grep / find / ps / kill


## Phase 2 — Virtual FileSystem（世界基础）

- [ ] Core File System
   - [ ] CRUD Files
      - [ ] DoD: 文件可创建/读取/修改/删除
   - [ ] CRUD Directories
      - [ ] DoD: 支持嵌套目录
   - [ ] Path Resolver
      - [ ] DoD: 支持 / .. .

- [ ] File Model
   - [ ] Metadata（name/type/size/owner/permission）
   - [ ] Directory Tree
      - [ ] DoD: 树结构稳定

- [ ] Permission System
   - [ ] DoD: 无权限无法操作

- [ ] Symlink（重要）
   - [ ] DoD: 支持链接（玩法用途）

- [ ] Stat System
   - [ ] DoD: 可查看文件信息

- [ ] 延后实现
   - [ ] ACL


## Phase 3 — Bit System（核心机制）

- [ ] Bit Attribute
   - [ ] File Size
      - [ ] DoD: 每个文件有 bit 大小
   - [ ] Directory Size
      - [ ] DoD: 目录 = 子项总和

- [ ] Player Bit Capacity
   - [ ] 定义容量
   - [ ] Usage Tracking
      - [ ] DoD: 实时显示使用量

- [ ] Memory Model（核心）
   - [ ] /player/memory/
   - [ ] Load Rule
   - [ ] Evict Rule
   - [ ] Clear Rule

- [ ] Overflow System
   - [ ] DoD: 超出容量报错

- [ ] Replace Strategy
   - [ ] DoD: 支持替换（LRU或手动）

- [ ] 延后实现
   - [ ] Stream Rule


## Phase 4 — Battle System（基础战斗）

- [ ] Entity Model
   - [ ] monster/
      - [ ] hp
      - [ ] atk
   - [ ] DoD: 可被操作

- [ ] User Model
   - [ ] /home/player/
   - [ ] /home/.status/
   - [ ] DoD: 状态以文件存在

- [ ] Attack System
   - [ ] DoD: 可修改 hp

- [ ] Turn System
   - [ ] DoD: 输入=回合

- [ ] Status System
   - [ ] DoD: 每回合生效


## Phase 5 — AI System（核心玩法）

- [ ] AI Script
   - [ ] monster/ai.sh

- [ ] Script Engine
   - [ ] attack
   - [ ] if
   - [ ] sleep

- [ ] Execution Loop
   - [ ] DoD: 每回合执行

- [ ] Player Interaction
   - [ ] cat ai.sh
   - [ ] chmod ai.sh
   - [ ] rm ai.sh
   - [ ] 修改 ai.sh

- [ ] DoD（关键）
   - [ ] 玩家可让敌人停止攻击


## Phase 6 — Permission & Control（战斗深化）

- [ ] Permission in Battle
   - [ ] hp 只读
   - [ ] chmod 破防

- [ ] Ownership（简化）
   - [ ] DoD: owner 影响权限


## Phase 7 — Root System（规则层）

- [ ] System Directories
   - [ ] /etc/
   - [ ] /proc/
   - [ ] /var/
   - [ ] /root/

- [ ] Rule System
   - [ ] /etc/rules/
      - [ ] DoD: 可修改规则

- [ ] sudo System（简化）
   - [ ] DoD: 临时越权

- [ ] 限制机制
   - [ ] bit 生效
   - [ ] 日志记录
   - [ ] 副作用（可选）


## Phase 8 — World System（扩展）

- [ ] Area System
   - [ ] /area/forest/
   - [ ] /area/city/

- [ ] Navigation
   - [ ] DoD: 可切换区域

- [ ] 延后实现
   - [ ] ssh / scp
   - [ ] docker / container
   - [ ] network / firewall / port


## Phase 9 — UX / Polish（体验优化）

- [ ] Bit UI
   - [ ] DoD: 显示使用量

- [ ] Error UX
   - [ ] DoD: 提示清晰

- [ ] Command Hint（可选）
   - [ ] 自动提示


## 最终目标（Demo 验收）

- [ ] 探索目录
- [ ] 查看敌人（受 bit 限制）
- [ ] 战斗
- [ ] 修改 AI
- [ ] chmod 破防

## 核心成功标志

- [ ] 玩家通过修改 ai.sh 改变敌人行为

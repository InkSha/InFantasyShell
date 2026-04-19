# InFantasyShell TODO (v1)

## Phase 1 — Runtime & Shell（基础运行环境）

- [ ] Bash-like Command Line（POSIX-like 子集）
   - [ ] Core
      - [ ] Prompt（带路径 + bit 信息） - In Progress
         - [x] DoD: 显示 player@world:/area$ [32B/64B]
      - [x] Parse Input（基础解析器）
         - [x] DoD: 能解析 command + args（如 cat a/b）
      - [x] Execute Command（命令调度）
         - [x] DoD: 命令正确路由执行
      - [x] Print Output（标准输出）
         - [x] DoD: 输出格式统一
      - [ ] Handle Errors（错误系统） - In Progress
         - [ ] DoD: 统一错误（not found / permission / overflow）

   - [x] Minimal Command Set
      - [x] cd
         - [x] DoD: 支持相对/绝对路径
      - [x] ls
         - [x] DoD: 列出目录内容
      - [x] pwd
         - [x] DoD: 显示当前路径
      - [x] cat
         - [x] DoD: 读取文件（受 bit 限制）
      - [x] echo
         - [x] DoD: 支持写入文件（>）
      - [x] rm
         - [x] DoD: 删除文件/目录
      - [x] chmod
         - [x] DoD: 修改权限

   - [ ] 延后实现（v2）
      - [ ] piping（|）
      - [ ] redirection（完整）
      - [ ] background jobs（&）
      - [ ] grep / find / ps / kill


## Phase 2 — Virtual FileSystem（世界基础）

- [x] Core File System
   - [x] CRUD Files
      - [x] DoD: 文件可创建/读取/修改/删除
   - [x] CRUD Directories
      - [x] DoD: 支持嵌套目录
   - [x] Path Resolver
      - [x] DoD: 支持 / .. .

- [x] File Model
   - [x] Metadata（name/type/size/owner/permission）
   - [x] Directory Tree
      - [x] DoD: 树结构稳定

- [x] Permission System
   - [x] DoD: 无权限无法操作

- [ ] Symlink（重要）
   - [ ] DoD: 支持链接（玩法用途）

- [x] Stat System
   - [x] DoD: 可查看文件信息

- [ ] 延后实现
   - [ ] ACL


## Phase 3 — Bit System（核心机制）

- [x] Bit Attribute
   - [x] File Size
      - [x] DoD: 每个文件有 bit 大小
   - [x] Directory Size
      - [x] DoD: 目录 = 子项总和

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

- [ ] Entity Model - In Progress
   - [ ] monster/
      - [x] hp
      - [ ] atk
   - [ ] DoD: 可被操作

- [x] User Model
   - [x] /home/player/
   - [x] /home/.status/
   - [x] DoD: 状态以文件存在

- [ ] Attack System
   - [ ] DoD: 可修改 hp

- [ ] Turn System
   - [ ] DoD: 输入=回合

- [ ] Status System
   - [ ] DoD: 每回合生效


## Phase 5 — AI System（核心玩法）

- [x] AI Script
   - [x] monster/ai.sh

- [ ] Script Engine
   - [ ] attack
   - [ ] if
   - [ ] sleep

- [ ] Execution Loop
   - [ ] DoD: 每回合执行

- [x] Player Interaction
   - [x] cat ai.sh
   - [x] chmod ai.sh
   - [x] rm ai.sh
   - [x] 修改 ai.sh

- [ ] DoD（关键）
   - [ ] 玩家可让敌人停止攻击


## Phase 6 — Permission & Control（战斗深化）

- [x] Permission in Battle
   - [x] hp 只读
   - [x] chmod 破防

- [x] Ownership（简化）
   - [x] DoD: owner 影响权限


## Phase 7 — Root System（规则层）

- [x] System Directories
   - [x] /etc/
   - [x] /proc/
   - [x] /var/
   - [x] /root/

- [x] Rule System
   - [x] /etc/rules/
      - [x] DoD: 可修改规则

- [ ] sudo System（简化）
   - [ ] DoD: 临时越权

- [x] 限制机制
   - [x] bit 生效
   - [x] 日志记录
   - [x] 副作用（可选）


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

- [x] 探索目录
- [x] 查看敌人（受 bit 限制）
- [x] 战斗
- [x] 修改 AI
- [x] chmod 破防

## 核心成功标志

- [ ] 玩家通过修改 ai.sh 改变敌人行为

export default {
  // 通用
  common: {
    confirm: '确认',
    cancel: '取消',
    save: '保存',
    delete: '删除',
    edit: '编辑',
    close: '关闭',
    browse: '浏览',
    select: '选择',
    clear: '清除',
    create: '创建',
    update: '更新',
    loading: '加载中...',
    success: '成功',
    error: '错误',
    warning: '警告',
    info: '信息',
    yes: '是',
    no: '否',
    ok: '确定',
    retry: '重试',
    reset: '重置',
    export: '导出',
    import: '导入',
    search: '搜索',
    settings: '设置',
    about: '关于',
    name: '名称',
    description: '描述',
    path: '路径',
    type: '类型',
    size: '大小',
    optional: '可选',
    required: '必填',
    version: '版本',
    allFiles: '所有文件',
    executableFiles: '可执行文件',
    scriptFiles: '脚本文件',
    shortcutFiles: '快捷方式',
    imageFiles: '图片文件',
    iconFiles: '图标文件',
    selectFile: '选择文件',
    selectFolder: '选择文件夹',
    userCancelled: '用户取消'
  },

  // 应用标题
  app: {
    title: 'Lora',
    loading: '正在启动 Lora...',
    cannotAddFile: '无法添加文件 {path}'
  },

  // 主窗口
  main: {
    title: 'Lora',
    search: {
      placeholder: '搜索应用和开始菜单...',
      results: '找到 {count} 个结果',
      startMenuIncluded: '其中 {count} 个来自开始菜单',
      loadingStartMenu: '正在载入开始菜单',
      hint: '• 按 Enter 启动第一个 • 按 ESC 退出搜索'
    },
    sidebar: {
      allApps: '全部应用'
    },
    tooltip: {
      appInfo: '启动项目：{name}\n使用频率：{count} 次\n上次启动：{lastLaunch}',
      startMenuInfo: '开始菜单项目：{name}\n点击即可直接启动',
      neverLaunched: '从未启动'
    },
    contextMenu: {
      newCategory: '新建分组',
      rename: '重命名',
      delete: '删除',
      deleteAll: '删除全部分组',
      runAsAdmin: '管理员权限运行',
      openFileLocation: '打开文件位置',
      copyFullPath: '复制完整路径',
      moveTo: '移动到',
      copyTo: '复制到',
      editApp: '编辑应用',
      deleteApp: '删除应用',
      deleteAllApps: '删除全部'
    },
    dialog: {
      renameCategory: '重命名分组',
      newName: '请输入新名称',
      editApp: '编辑应用',
      appName: '应用名称',
      category: '所属分组',
      targetPath: '目标路径',
      launchArgs: '启动参数 (可选)',
      icon: '图标 (可选)',
      selectIcon: '选择图标',
      clearIcon: '清除图标',
      targetType: '目标类型',
      file: '文件',
      folder: '文件夹',
      url: '网址',
      runAsAdmin: '始终使用管理员打开',
      filePlaceholder: '请输入文件路径',
      folderPlaceholder: '请输入文件夹路径',
      urlPlaceholder: '请输入网址，如: https://example.com',
      launchArgsPlaceholder: '请输入启动参数 (如: --fullscreen --debug)'
    },
    toast: {
      pathCopied: '路径已复制到剪贴板',
      appUpdated: '应用信息已更新',
      preventAutoHideEnabled: '已启用阻止自动隐藏',
      preventAutoHideDisabled: '已禁用阻止自动隐藏',
      settingsUpdateFailed: '设置更新失败',
      openSettingsFailed: '打开设置窗口失败',
      openNewProjectFailed: '打开新建项目窗口失败',
      openEditProjectFailed: '打开编辑项目窗口失败'
    },
    drag: {
      message: '拖拽程序文件到这里添加到启动器'
    },
    confirm: {
      deleteApp: '确定要删除应用 "{name}" 吗？',
      deleteAllApps: '确定要删除当前分类下的所有应用吗？',
      deleteCategory: '确定要删除分组 "{name}" 吗？这将同时删除该分组下的 {count} 个应用。',
      deleteCategoryEmpty: '确定要删除分组 "{name}" 吗？',
      deleteAllCategories: '确定要删除所有 {groupCount} 个自定义分组吗？这将同时删除 {appCount} 个应用。',
      deleteAllCategoriesEmpty: '确定要删除所有 {groupCount} 个自定义分组吗？',
      exit: '退出'
    },
    alert: {
      appPathNotExist: '应用路径不存在，无法启动',
      launchFailed: '启动应用失败: {error}',
      deleteAppFailed: '删除应用失败',
      runAsAdminFailed: '以管理员权限运行失败: {error}',
      openFileLocationFailed: '打开文件位置失败: {error}',
      browseFileFailed: '浏览文件失败: {error}',
      selectIconFailed: '选择图标失败: {error}',
      noCustomCategories: '没有自定义分组可以删除。'
    }
  },

  // 设置窗口
  settings: {
    title: '设置',
    tabs: {
      about: '关于',
      ui: '界面设置',
      features: '功能设置'
    },
    about: {
      appName: 'Lora Launcher',
      description: '一个简洁高效的应用启动器，让您快速访问常用应用程序。',
      developer: '开发者',
      team: 'Lora Team',
      techStack: '技术栈',
      technologies: 'Tauri + Vue 3',
      license: '许可证',
      mitLicense: 'MIT License',
      updateDate: '更新日期',
      links: {
        github: 'GitHub 仓库',
        reportIssue: '报告问题',
        checkUpdate: '检查更新'
      }
    },
    ui: {
      window: {
        title: '窗口设置',
        width: '窗口宽度',
        height: '窗口高度',
        layout: '窗口布局',
        layoutHorizontal: '横向布局',
        layoutVertical: '竖向布局',
        layoutLocked: '锁定启动器布局',
        layoutLockedDesc: '禁止调整侧栏宽度以及拖动应用或分组排序，其他功能不受影响',
        preventAutoHide: '阻止窗口自动隐藏',
        preventAutoHideDesc: '启用此选项后，窗口失去焦点时不会自动隐藏'
      },
      appearance: {
        title: '外观设置',
        theme: '主题',
        auto: '自动',
        light: '浅色',
        dark: '深色',
        iconSize: '应用图标大小',
        projectNamePosition: '项目名称显示位置',
        namePositionNone: '无',
        namePositionTop: '上',
        namePositionBottom: '下',
        namePositionLeft: '左',
        namePositionRight: '右',
        gridCellSizeHint: '可在启动器窗口中按住 Ctrl 并滚动鼠标滚轮调整格子大小。',
        small: '小',
        medium: '中',
        large: '大',
        sidebarWidth: '侧栏宽度'
      },
      animation: {
        title: '动画设置',
        enableAnimations: '启用动画效果',
        enableAnimationsDesc: '禁用动画可以提高性能，特别是在低配置设备上',
        speed: '动画速度',
        slow: '慢速',
        normal: '正常',
        fast: '快速'
      },
      categories: {
        title: '分组显示',
        description: '隐藏分组后，它不会出现在左侧侧栏；分组内应用仍会出现在全部应用和搜索结果中。',
        visible: '显示',
        hidden: '隐藏',
        empty: '暂无自定义分组'
      }
    },
    features: {
      startup: {
        title: '启动设置',
        startWithSystem: '开机自动启动',
        startWithSystemDesc: '将 Lora 添加到系统启动项，开机时自动运行',
        startMinimized: '启动时最小化到托盘',
        startMinimizedDesc: '程序启动时直接最小化到系统托盘，不显示主窗口',
        autoHideAfterLaunch: '运行应用后自动隐藏',
        autoHideAfterLaunchDesc: '启动应用后自动隐藏启动器窗口，避免占用屏幕空间'
      },
      hotkey: {
        title: '快捷键设置',
        toggleWindow: '显示/隐藏窗口',
        setHotkey: '点击设置快捷键',
        clear: '清除',
        enableGlobalHotkey: '启用全局快捷键',
        enableGlobalHotkeyDesc: '在任何窗口下都能使用快捷键唤起 Lora',
        middleMouseToggle: '使用鼠标中键显示或隐藏 Lora',
        middleMouseToggleDesc: '在任意位置按下鼠标中键即可切换窗口，原有中键操作仍会正常执行'
      },
      search: {
        title: '搜索设置',
        enableFuzzy: '启用模糊搜索',
        enableFuzzyDesc: '允许不完全匹配的搜索结果，提高搜索的容错性',
        searchInPath: '搜索文件路径',
        searchInPathDesc: '在搜索时包含文件路径信息',
        maxResults: '最大搜索结果数',
        results: '{count} 个',
        sortOrder: '应用排序方式',
        sortManual: '手动排序',
        sortName: '按名称排序',
        sortFrequency: '按使用频率排序',
        sortOrderDesc: '使用频率会在应用成功启动后自动累计'
      },
      data: {
        title: '数据管理',
        autoBackup: '自动备份数据',
        autoBackupDesc: '定期自动备份应用数据，防止数据丢失',
        backupInterval: '备份频率',
        daily: '每天',
        weekly: '每周',
        monthly: '每月',
        exportData: '导出数据',
        importData: '导入数据',
        resetData: '重置数据'
      }
    },
    footer: {
      saving: '正在保存...',
      saved: '已保存',
      restoreDefaults: '恢复默认',
      saveSettings: '保存设置'
    },
    confirm: {
      resetSettings: '确定要恢复所有设置到默认值吗？',
      resetData: '确定要重置所有数据吗？此操作不可撤销！'
    },
    alert: {
      exportSuccess: '数据导出成功！',
      exportFailed: '导出数据失败: {error}',
      importSuccess: '数据导入成功！请重启应用以应用更改。',
      importFailed: '导入数据失败: {error}',
      resetSuccess: '数据重置成功！请重启应用。',
      resetFailed: '重置数据失败: {error}',
      restoreDefaultsSuccess: '设置已恢复到默认值',
      restoreDefaultsFailed: '恢复默认设置失败: {error}',
      updateCategoryVisibilityFailed: '更新分组显示状态失败: {error}'
    }
  },

  // 新建项目窗口
  newProject: {
    title: '新建项目',
    projectName: '项目名称',
    projectNamePlaceholder: '请输入项目名称',
    category: '所属分组',
    targetType: '目标类型',
    targetPath: '目标路径',
    url: '网址',
    targetTypeFile: '文件',
    targetTypeFolder: '文件夹',
    targetTypeUrl: '网址',
    filePathPlaceholder: '请输入文件路径',
    folderPathPlaceholder: '请输入文件夹路径',
    urlPlaceholder: '请输入网址，如: https://example.com',
    projectIcon: '项目图标',
    launchArgs: '启动参数',
    launchArgsPlaceholder: '请输入启动参数 (如: --fullscreen --debug)',
    projectHotkey: '项目快捷键',
    projectHotkeyPlaceholder: '按下组合键（至少包含一个修饰键）',
    runAsAdmin: '始终使用管理员打开',
    projectDescription: '项目描述',
    projectDescriptionPlaceholder: '请输入项目描述',
    allFiles: '所有文件',
    executableFiles: '可执行文件',
    scriptFiles: '脚本文件',
    shortcutFiles: '快捷方式',
    imageFiles: '图片文件',
    iconFiles: '图标文件',
    selectTargetFile: '选择目标文件',
    selectTargetFolder: '选择目标文件夹',
    selectIconFile: '选择图标文件',
    userCancelled: '用户取消',
    browseFileError: '浏览文件失败',
    selectIconError: '选择图标失败',
    fillRequiredFields: '请填写必要的信息',
    createProjectError: '创建项目失败',
    creating: '正在创建...',
    created: '已创建',
    customProject: '自定义',
    builtInItems: '内置项目',
    startMenuItems: '开始菜单',
    selectStartMenuShortcut: '选择开始菜单项目',
    selectStartMenuShortcutError: '选择开始菜单项目失败',
    noStartMenuItems: '未找到开始菜单项目',
    searchStartMenuPlaceholder: '搜索开始菜单项目或路径...',
    createProject: '创建项目',
    presets: {
      computer: '计算机',
      network: '网络连接',
      recycleBin: '回收站',
      volumeMixer: '音量合成器',
      calculator: '计算器',
      registryEditor: '注册表编辑器',
      groupPolicy: '组策略',
      controlPanel: '控制面板',
      taskManager: '任务管理器',
      cmd: '命令提示符',
      powershell: 'PowerShell',
      notepad: '记事本',
      paint: '画图',
      wordpad: '写字板',
      services: '服务',
      deviceManager: '设备管理器',
      diskManagement: '磁盘管理',
      eventViewer: '事件查看器',
      systemInfo: '系统信息',
      remoteDesktop: '远程桌面连接',
      snippingTool: '截图工具',
      characterMap: '字符映射表',
      magnifier: '放大镜',
      osk: '屏幕键盘',
      runDialog: '运行',
      downloads: '下载',
      documents: '文档',
      pictures: '图片',
      shutdown: '关机',
      restart: '重启',
      sleep: '睡眠',
      startMenuPrograms: '开始菜单程序',
      commonStartMenuPrograms: '公共开始菜单',
      startupFolder: '启动文件夹',
      appsFolder: '应用列表'
    },
    form: {
      projectName: '项目名称',
      category: '所属分组',
      targetType: '目标类型',
      targetPath: '目标路径',
      targetTypeFile: '文件',
      targetTypeFolder: '文件夹',
      targetTypeUrl: '网址',
      url: '网址',
      projectIcon: '项目图标',
      launchArgs: '启动参数',
      projectHotkey: '项目快捷键',
      projectHotkeyPlaceholder: '按下组合键（至少包含一个修饰键）',
      runAsAdmin: '始终使用管理员打开',
      projectDescription: '项目描述',
      filePlaceholder: '请输入文件路径',
      folderPlaceholder: '请输入文件夹路径',
      urlPlaceholder: '请输入网址，如: https://example.com',
      launchArgsPlaceholder: '请输入启动参数 (如: --fullscreen --debug)',
      descriptionPlaceholder: '请输入项目描述',
      saveChanges: '保存'
    },
    footer: {
      cancel: '取消',
      create: '创建项目'
    },
    status: {
      creating: '正在创建...',
      created: '已创建'
    },
    confirm: {
      fillRequired: '请填写必要的信息'
    },
    alert: {
      createFailed: '创建项目失败: {error}',
      browseFileFailed: '浏览文件失败: {error}',
      selectIconFailed: '选择图标失败: {error}'
    }
  },

  // 编辑项目窗口
  editProject: {
    title: '编辑项目',
    filePathPlaceholder: '请输入文件路径',
    folderPathPlaceholder: '请输入文件夹路径',
    urlPlaceholder: '请输入网址，如: https://example.com',
    form: {
      projectName: '项目名称',
      projectNamePlaceholder: '请输入项目名称',
      category: '所属分组',
      targetType: '目标类型',
      targetPath: '目标路径',
      targetTypeFile: '文件',
      targetTypeFolder: '文件夹',
      targetTypeUrl: '网址',
      url: '网址',
      projectIcon: '项目图标',
      launchArgs: '启动参数',
      projectHotkey: '项目快捷键',
      projectHotkeyPlaceholder: '按下组合键（至少包含一个修饰键）',
      runAsAdmin: '始终使用管理员打开',
      projectDescription: '项目描述',
      filePlaceholder: '请输入文件路径',
      folderPlaceholder: '请输入文件夹路径',
      urlPlaceholder: '请输入网址，如: https://example.com',
      launchArgsPlaceholder: '请输入启动参数 (如: --fullscreen --debug)',
      descriptionPlaceholder: '请输入项目描述',
      projectDescriptionPlaceholder: '请输入项目描述',
      saveChanges: '保存更改'
    },
    footer: {
      cancel: '取消',
      save: '保存更改'
    },
    status: {
      loading: '正在加载数据...',
      saving: '正在保存...',
      saved: '已保存'
    },
    error: {
      loadFailed: '加载失败',
      loadAppDataFailed: '加载应用数据失败',
      noAppId: '未找到应用ID参数，请确保通过正确的方式打开编辑窗口',
      getAppFailed: '获取应用数据失败'
    },
    confirm: {
      fillRequired: '请填写必要的信息'
    },
    alert: {
      saveFailed: '保存项目失败: {error}',
      browseFileFailed: '浏览文件失败: {error}',
      selectIconFailed: '选择图标失败: {error}'
    }
  },

  // 语言切换
  language: {
    switch: '切换语言',
    chinese: '中文',
    english: 'English'
  }
}

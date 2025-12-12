export default {
  // 通用
  common: {
    confirm: 'Confirm',
    cancel: 'Cancel',
    save: 'Save',
    delete: 'Delete',
    edit: 'Edit',
    close: 'Close',
    browse: 'Browse',
    select: 'Select',
    clear: 'Clear',
    create: 'Create',
    update: 'Update',
    loading: 'Loading...',
    success: 'Success',
    error: 'Error',
    warning: 'Warning',
    info: 'Info',
    yes: 'Yes',
    no: 'No',
    ok: 'OK',
    retry: 'Retry',
    reset: 'Reset',
    export: 'Export',
    import: 'Import',
    search: 'Search',
    settings: 'Settings',
    about: 'About',
    name: 'Name',
    description: 'Description',
    path: 'Path',
    type: 'Type',
    size: 'Size',
    optional: 'Optional',
    required: 'Required',
    version: 'Version',
    allFiles: 'All Files',
    executableFiles: 'Executable Files',
    scriptFiles: 'Script Files',
    shortcutFiles: 'Shortcut Files',
    imageFiles: 'Image Files',
    iconFiles: 'Icon Files',
    selectFile: 'Select File',
    selectFolder: 'Select Folder',
    userCancelled: 'User Cancelled'
  },

  // 应用标题
  app: {
    title: 'Lora',
    loading: 'Starting Lora...'
  },

  // 主窗口
  main: {
    title: 'Lora',
    search: {
      placeholder: 'Search applications...',
      results: 'Found {count} results',
      hint: '• Press Enter to launch first • Press ESC to exit search'
    },
    sidebar: {
      allApps: 'All Apps'
    },
    contextMenu: {
      newCategory: 'New Category',
      rename: 'Rename',
      delete: 'Delete',
      deleteAll: 'Delete All Categories',
      runAsAdmin: 'Run as Administrator',
      openFileLocation: 'Open File Location',
      copyFullPath: 'Copy Full Path',
      moveTo: 'Move To',
      editApp: 'Edit App',
      deleteApp: 'Delete App',
      deleteAllApps: 'Delete All'
    },
    dialog: {
      renameCategory: 'Rename Category',
      newName: 'Please enter new name',
      editApp: 'Edit Application',
      appName: 'Application Name',
      category: 'Category',
      targetPath: 'Target Path',
      launchArgs: 'Launch Arguments (Optional)',
      icon: 'Icon (Optional)',
      selectIcon: 'Select Icon',
      clearIcon: 'Clear Icon',
      targetType: 'Target Type',
      file: 'File',
      folder: 'Folder',
      url: 'URL',
      filePlaceholder: 'Please enter file path',
      folderPlaceholder: 'Please enter folder path',
      urlPlaceholder: 'Please enter URL, e.g.: https://example.com',
      launchArgsPlaceholder: 'Please enter launch arguments (e.g.: --fullscreen --debug)'
    },
    toast: {
      pathCopied: 'Path copied to clipboard',
      appUpdated: 'Application information updated',
      preventAutoHideEnabled: 'Prevent auto-hide enabled',
      preventAutoHideDisabled: 'Prevent auto-hide disabled',
      settingsUpdateFailed: 'Failed to update settings',
      openSettingsFailed: 'Failed to open settings window',
      openNewProjectFailed: 'Failed to open new project window',
      openEditProjectFailed: 'Failed to open edit project window'
    },
    drag: {
      message: 'Drag program files here to add to launcher'
    },
    confirm: {
      deleteApp: 'Are you sure you want to delete application "{name}"?',
      deleteAllApps: 'Are you sure you want to delete all applications in the current category?',
      deleteCategory: 'Are you sure you want to delete category "{name}"? This will also delete {count} applications in this category.',
      deleteAllCategories: 'Are you sure you want to delete all {count} custom categories? This will also delete {count} applications.',
      exit: 'Exit'
    },
    alert: {
      appPathNotExist: 'Application path does not exist, cannot launch',
      launchFailed: 'Failed to launch application: {error}',
      deleteAppFailed: 'Failed to delete application',
      runAsAdminFailed: 'Failed to run as administrator: {error}',
      openFileLocationFailed: 'Failed to open file location: {error}',
      browseFileFailed: 'Failed to browse file: {error}',
      selectIconFailed: 'Failed to select icon: {error}',
      noCustomCategories: 'No custom categories available to delete.'
    }
  },

  // 设置窗口
  settings: {
    title: 'Settings',
    tabs: {
      about: 'About',
      ui: 'Interface Settings',
      features: 'Feature Settings'
    },
    about: {
      appName: 'Lora Launcher',
      description: 'A concise and efficient application launcher that lets you quickly access commonly used applications.',
      developer: 'Developer',
      team: 'Lora Team',
      techStack: 'Tech Stack',
      technologies: 'Tauri + Vue 3',
      license: 'License',
      mitLicense: 'MIT License',
      updateDate: 'Update Date',
      links: {
        github: 'GitHub Repository',
        reportIssue: 'Report Issue',
        checkUpdate: 'Check Update'
      }
    },
    ui: {
      window: {
        title: 'Window Settings',
        width: 'Window Width',
        height: 'Window Height',
        preventAutoHide: 'Prevent Window Auto-Hide',
        preventAutoHideDesc: 'When enabled, the window will not auto-hide when it loses focus'
      },
      appearance: {
        title: 'Appearance Settings',
        theme: 'Theme',
        auto: 'Auto',
        light: 'Light',
        dark: 'Dark',
        iconSize: 'Application Icon Size',
        sidebarWidth: 'Sidebar Width'
      },
      animation: {
        title: 'Animation Settings',
        enableAnimations: 'Enable Animation Effects',
        enableAnimationsDesc: 'Disabling animations can improve performance, especially on low-end devices',
        speed: 'Animation Speed',
        slow: 'Slow',
        normal: 'Normal',
        fast: 'Fast'
      }
    },
    features: {
      startup: {
        title: 'Startup Settings',
        startWithSystem: 'Start with System',
        startWithSystemDesc: 'Add Lora to system startup items to run automatically when the system starts',
        startMinimized: 'Start Minimized to Tray',
        startMinimizedDesc: 'Start the application directly minimized to the system tray without showing the main window',
        autoHideAfterLaunch: 'Auto-hide After Launch',
        autoHideAfterLaunchDesc: 'Automatically hide the launcher window after launching an application to avoid taking up screen space'
      },
      hotkey: {
        title: 'Hotkey Settings',
        toggleWindow: 'Show/Hide Window',
        setHotkey: 'Click to set hotkey',
        clear: 'Clear',
        enableGlobalHotkey: 'Enable Global Hotkey',
        enableGlobalHotkeyDesc: 'Use hotkey to invoke Lora from any window'
      },
      search: {
        title: 'Search Settings',
        enableFuzzy: 'Enable Fuzzy Search',
        enableFuzzyDesc: 'Allow incomplete matching search results to improve search fault tolerance',
        searchInPath: 'Search File Paths',
        searchInPathDesc: 'Include file path information in search',
        maxResults: 'Maximum Search Results',
        results: '{count} items'
      },
      data: {
        title: 'Data Management',
        autoBackup: 'Auto Backup Data',
        autoBackupDesc: 'Periodically automatically backup application data to prevent data loss',
        backupInterval: 'Backup Frequency',
        daily: 'Daily',
        weekly: 'Weekly',
        monthly: 'Monthly',
        exportData: 'Export Data',
        importData: 'Import Data',
        resetData: 'Reset Data'
      }
    },
    footer: {
      saving: 'Saving...',
      saved: 'Saved',
      restoreDefaults: 'Restore Defaults',
      saveSettings: 'Save Settings'
    },
    confirm: {
      resetSettings: 'Are you sure you want to restore all settings to their default values?',
      resetData: 'Are you sure you want to reset all data? This operation cannot be undone!'
    },
    alert: {
      exportSuccess: 'Data exported successfully!',
      exportFailed: 'Failed to export data: {error}',
      importSuccess: 'Data imported successfully! Please restart the application to apply changes.',
      importFailed: 'Failed to import data: {error}',
      resetSuccess: 'Data reset successfully! Please restart the application.',
      resetFailed: 'Failed to reset data: {error}',
      restoreDefaultsSuccess: 'Settings restored to default values',
      restoreDefaultsFailed: 'Failed to restore default settings: {error}'
    }
  },

  // 新建项目窗口
  newProject: {
    title: 'New Project',
    projectName: 'Project Name',
    projectNamePlaceholder: 'Please enter project name',
    category: 'Category',
    targetType: 'Target Type',
    targetPath: 'Target Path',
    url: 'URL',
    targetTypeFile: 'File',
    targetTypeFolder: 'Folder',
    targetTypeUrl: 'URL',
    filePathPlaceholder: 'Please enter file path',
    folderPathPlaceholder: 'Please enter folder path',
    urlPlaceholder: 'Please enter URL, e.g.: https://example.com',
    projectIcon: 'Project Icon',
    launchArgs: 'Launch Arguments',
    launchArgsPlaceholder: 'Please enter launch arguments (e.g.: --fullscreen --debug)',
    projectDescription: 'Project Description',
    projectDescriptionPlaceholder: 'Please enter project description',
    allFiles: 'All Files',
    createProject: 'Create Project',
    form: {
      projectName: 'Project Name',
      category: 'Category',
      targetType: 'Target Type',
      targetPath: 'Target Path',
      url: 'URL',
      targetTypeFile: 'File',
      targetTypeFolder: 'Folder',
      targetTypeUrl: 'URL',
      projectIcon: 'Project Icon',
      launchArgs: 'Launch Arguments',
      projectDescription: 'Project Description',
      filePlaceholder: 'Please enter file path',
      folderPlaceholder: 'Please enter folder path',
      urlPlaceholder: 'Please enter URL, e.g.: https://example.com',
      launchArgsPlaceholder: 'Please enter launch arguments (e.g.: --fullscreen --debug)',
      descriptionPlaceholder: 'Please enter project description',
      saveChanges: 'Save'
    },
    footer: {
      cancel: 'Cancel',
      create: 'Create Project'
    },
    status: {
      creating: 'Creating...',
      created: 'Created'
    },
    confirm: {
      fillRequired: 'Please fill in the required information'
    },
    alert: {
      createFailed: 'Failed to create project: {error}',
      browseFileFailed: 'Failed to browse file: {error}',
      selectIconFailed: 'Failed to select icon: {error}'
    }
  },

  // 编辑项目窗口
  editProject: {
    title: 'Edit Project',
    filePathPlaceholder: 'Please enter file path',
    folderPathPlaceholder: 'Please enter folder path',
    urlPlaceholder: 'Please enter URL, e.g.: https://example.com',
    form: {
      projectName: 'Project Name',
      projectNamePlaceholder: 'Please enter project name',
      category: 'Category',
      targetType: 'Target Type',
      targetPath: 'Target Path',
      targetTypeFile: 'File',
      targetTypeFolder: 'Folder',
      targetTypeUrl: 'URL',
      url: 'URL',
      projectIcon: 'Project Icon',
      launchArgs: 'Launch Arguments',
      projectDescription: 'Project Description',
      filePlaceholder: 'Please enter file path',
      folderPlaceholder: 'Please enter folder path',
      urlPlaceholder: 'Please enter URL, e.g.: https://example.com',
      launchArgsPlaceholder: 'Please enter launch arguments (e.g.: --fullscreen --debug)',
      descriptionPlaceholder: 'Please enter project description',
      projectDescriptionPlaceholder: 'Please enter project description',
      saveChanges: 'Save Changes'
    },
    footer: {
      cancel: 'Cancel',
      save: 'Save Changes'
    },
    status: {
      loading: 'Loading data...',
      saving: 'Saving...',
      saved: 'Saved'
    },
    error: {
      loadFailed: 'Loading Failed',
      loadAppDataFailed: 'Failed to load application data',
      noAppId: 'Application ID parameter not found, please ensure the edit window is opened correctly',
      getAppFailed: 'Failed to get application data'
    },
    confirm: {
      fillRequired: 'Please fill in the required information'
    },
    alert: {
      saveFailed: 'Failed to save project: {error}',
      browseFileFailed: 'Failed to browse file: {error}',
      selectIconFailed: 'Failed to select icon: {error}'
    }
  },

  // 语言切换
  language: {
    switch: 'Switch Language',
    chinese: '中文',
    english: 'English'
  }
}
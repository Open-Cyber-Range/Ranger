const language = {
  homepage: 'Welcome to Ranger!',
  appName: 'Ranger',
  OCR: 'Open Cyber Range',
  documentation: 'Documentation',
  create: 'Create',
  update: 'Update',
  delete: 'Delete',
  exercises: {
    title: 'Exercises',
    add: 'Add exercise',
    name: 'Exercise name',
    scenarioSDL: 'Scenario SDL',
    sdlGuide: 'SDL Reference Guide',
    sdlParserInitFail: 'SDL Parser Initialization Failed',
    sdlParsingFail: 'SDL Parsing failed',
    sdlNotSaved: 'You have unsaved changes in SDL',
    addingSuccess: 'Exercise {{exerciseName}} added',
    addingFail: 'Failed to add the exercise',
    updateSuccess: 'Exercise {{exerciseName}} updated',
    updateFail: 'Failed to update the exercise: {{errorMessage}}',
    noBanner: 'No banner exists for this exercise',
    noDeployments: 'No deployments',
    noDeploymentInfo: 'No deployment info',
    deleteSuccess: 'Exercise {{exerciseName}} deleted',
    deleteFail: 'Failed to delete the exercise {{exerciseName}}',
    easeDevelopment: 'To ease the development of the exercise open: ',
    mustHaveName: 'Exercise must have a name',
    group: {
      placeholder: 'Select group...',
      title: 'Deployment group',
      required: 'Deployment group is required',
    },
    tabs: {
      dashboard: 'Dashboard',
      banner: 'Banner',
      scores: 'Scores',
      deploymentScores: 'Deployment Scores',
      emails: 'Emails',
      emailLogs: 'Email Activity',
      accounts: 'Accounts',
      entities: 'Entity Selector',
      userSubmissions: 'User Submissions',
      sdl: 'SDL',
      events: 'Events',
    },
    estimatedResourcesTitle: 'Estimated resources:',
    estimatedResources: 'Total RAM: {{totalRam}}, total CPUs: {{totalCpu}}',
    estimatedResourcesFail: 'Resource estimation failed',
  },
  deployments: {
    title: 'Deployments',
    add: 'Add new',
    create: 'Create a new deployment',
    addingSuccess: 'Deployment {{newDeploymentName}} added',
    addingFail: 'Failed to add the deployment',
    sdlMissing: 'Exercise must have an sdl-schema',
    deleteSuccess: 'Deployment {{deploymentName}} deleted',
    deleteFail: 'Failed to delete the deployment',
    noDeployments: 'No deployments',
    noAccounts: 'No accounts',
    entitySelect: 'Select an entity',
    entityConnector: {
      entityConnector: 'Entity Connector',
      selectEntity: 'Select entity...',
      selectUser: 'Select user...',
      success: 'Entity connected successfully',
      fail: 'Failed to connect entity',
    },
    entityTree: 'Entity Tree',
    form: {
      name: {
        title: 'Deployment(s) name',
        required: 'Deployment(s) name is required',
      },
      count: {
        title: 'Number of deployments',
        required: 'Number of deployments is required',
      },
      startDate: {
        title: 'Deployment(s) start time',
        required: 'Deployment(s) start time is required',
      },
      adGroups: {
        title: 'AD Group for deployment #{{number}}',
      },
      endDate: {
        title: 'Deployment(s) end time',
        required: 'Deployment(s) end time is required',
        earlierThanStart: 'Deployment(s) end time must be later than start time',
      },
    },
    beingDeployed: 'Deployment can not be deleted while it is being deployed',
    beingDeleted: 'Deployment is being deleted',
    events: {
      title: 'Events',
      startTime: 'Start time:',
      endTime: 'End time:',
      description: 'Description:',
      nodes: 'Nodes:',
      noEventsYet: 'No Events to display - deployment in progress',
      noScenarioEvents: 'No Events to display - scenario has no events',
      noTriggeredEvents: 'No Events have been triggered yet',
      noDescription: 'Event has no description',
      notTriggered: 'Event has not been triggered',
      triggeredAt: 'Triggered at {{date}}',
      eventWindowClosed: 'Event window closed',
      eventWillOpenIn: 'Event window will open in',
      eventWillCloseIn: 'Event window will close in ',
      allNodesHaveTriggered: 'All nodes have triggered the event',
    },
    status: {
      showStatusBox: 'Show Deployment Element Statuses',
      hideStatusBox: 'Hide Deployment Element Statuses',
      title: 'Deployment Status',
      success: 'Operation Successful',
      ongoing: 'Operation Ongoing',
      failed: 'Operation Failed',
      removed: 'Element Removed',
      removeFailed: 'Element Removal Failed',
      conditionSuccess: 'Condition Met',
      conditionPolling: 'Condition Checking Ongoing',
      conditionClosed: 'Condition Checking Closed',
      conditionWarning: 'Condition Warning',
      unknown: 'Unknown Status',
      cardFields: {
        handlerReference: 'Handler Reference:',
        type: 'Type:',
        status: 'Status:',
        errorMessage: 'Error Message:',
        stdoutLogs: 'Stdout Logs:',
        stderrLogs: 'Stderr Logs:',
        showErrorMessage: 'Show Error Message',
        hideErrorMessage: 'Hide Error Message',
        showStdoutLogs: 'Show Stdout Logs',
        hideStdoutLogs: 'Hide Stdout Logs',
        showStderrLogs: 'Show Stderr Logs',
        hideStderrLogs: 'Hide Stderr Logs',
      },
    },
    deployerTypes: {
      switch: 'Switch',
      switches: 'Switches',
      template: 'Template',
      templates: 'Templates',
      virtualMachine: 'Virtual Machine',
      virtualMachines: 'Virtual Machines',
      feature: 'Feature',
      features: 'Features',
      condition: 'Condition',
      conditions: 'Conditions',
      inject: 'Inject',
      injects: 'Injects',
      unknown: 'Unknown',
    },
  },
  menu: {
    home: 'Home',
    exercises: 'Exercises',
    logout: 'Logout',
    greeting: 'Hi, {{username}}!',
    userRoles: {
      'ranger-manager': 'Manager',
      'ranger-participant': 'Participant',
      'ranger-client': 'Client',
    },
    noRole: 'NO ROLE',
    logs: 'Logs',
  },
  common: {
    connect: 'Connect',
    required: '(required)',
    submit: 'Submit',
    delete: 'Delete',
    back: 'Back',
    deleting: 'Deleting',
    add: 'Add',
    virtualMachines: 'Virtual Machines',
    switches: 'Switches',
    templates: 'Templates',
    team: 'Team',
    noResults: 'No results',
    adGroup: 'AD Group',
    browse: 'Browse',
    points: 'Points',
    collapse: 'Collapse',
    expand: 'Expand',
  },
  emails: {
    status: 'Status',
    timestamp: 'Timestamp',
    from: 'From',
    to: 'To',
    cc: 'Cc',
    bcc: 'Bcc',
    replyTo: 'Reply to',
    subject: 'Subject',
    body: 'Body',
    sendingSuccess: 'Email sent',
    sendingFail: 'Error trying to send the email: {{errorMessage}}',
    sendingFailWithoutMessage: 'Failed to send the email',
    invalidEmailAddress: 'Invalid email address(es): {{invalidEmailAddresses}}',
    noDeployment: 'Cannot find this deployment',
    noDeployments: 'Cannot find any deployments for this exercise',
    fetchingUsersFail: 'Failed to fetch users',
    creatingEmailsFail: 'Failed to create emails',
    addingTemplateSuccess: 'Email template added',
    addingTemplateFail: 'Failed to add email template {{errorMessage}}',
    addingTemplateFailWithoutMessage: 'Failed to add email template',
    deletingTemplateSuccess: 'Email template deleted',
    deletingTemplateFail: 'Failed to delete email template {{errorMessage}}',
    deletingTemplateFailWithoutMessage: 'Failed to delete email template',
    form: {
      from: {
        title: 'From',
      },
      deploymentSelector: {
        title: 'Selected deployment',
        placeholder: 'Select exercise or specific deployment',
        wholeExercise: 'Exercise-wide',
      },
      to: {
        title: 'To',
        required: 'To address is required',
      },
      replyTo: {
        title: 'Reply to',
      },
      cc: {
        title: 'Cc',
      },
      bcc: {
        title: 'Bcc',
      },
      subject: {
        title: 'Email subject',
        required: 'Email subject is required',
      },
      templateName: {
        name: 'Email template name',
        title: 'Email template',
        placeholder: 'Select existing email template',
        required: 'Email template name is required',
        save: 'Save body as template',
        delete: 'Delete selected template',
      },
      body: {
        title: 'Email body (HTML text editor)',
        required: 'Email body is required',
      },
      preview: 'Preview',
      send: 'Send',
      sendButtonDisabled: 'Fetching users to send emails to...',
      emailPlaceholder:
      '...multiple addresses separated by comma or enter',
      required: ' (required)',
    },
    variables: {
      available: 'Available variables:',
      insert: 'Insert variable',
      exerciseName: 'Exercise\'s name',
      deploymentName: 'Deployment\'s name',
      participantFirstName: 'Participant\'s first name',
      participantLastName: 'Participant\'s last name',
      participantEmail: 'Participant\'s email',
      username: 'Participant\'s username',
    },
    fetchingEmails: 'Fetching emails...',
    fetchingEmailsFail: 'Failed to fetch emails',
    noEmails: 'No emails to display',
    viewBody: 'View email body',
  },
  tloTable: {
    headers: {
      tlo: 'Training Learning Objective',
      evaluation: 'Evaluation',
      metrics: 'Metrics',
      name: 'Name',
      vm: 'VM',
      points: 'Points',
    },
    evaluation: {
      minScore: 'Min score',
      passed: 'Passed',
      notMet: 'Not met',
    },
    noEvaluations: 'No Evaluations to display',
    noTlos: 'No Training Learning Objectives to display',
    noMetricData: 'No metric scores to display',
  },
  chart: {
    scoring: {
      title: 'Score',
      xAxisTitle: 'Time',
      yAxisTitle: 'Points',
      noScoreData: 'No score data available',
    },
  },
  participant: {
    exercise: {
      tabs: {
        dash: 'Dashboard',
        score: 'Score',
        events: 'Events',
        accounts: 'Accounts',
        userSubmissions: 'User Submissions',
      },
      events: {
        noEvents: 'Participant is not connected to any Events in this deployment',
        noTriggeredEvents: 'No Events have been triggered yet',
        noDescription: 'Event has no additional description',
        triggeredAt: 'Event triggered at: {{date}}',
      },
    },
  },
  metricScoring: {
    score: 'Score',
    noManualMetrics: 'No manual metric submissions to score at the moment',
    downloadButtonLoading: 'Downloading...',
    downloadButton: 'Download File',
    textSubmissionPlaceholder: 'No text submission yet...',
    scorePlaceholder: 'Enter a score...',
    updateSuccess: 'Metric successfully updated',
    newSuccess: 'Metric submission successfully created',
    artifactAdded: 'Artifact successfully added',
    notScored: 'Not yet been scored',
    addArtifactPlaceholder: 'Add your artifact...',
    replaceArtifactPlaceholder: 'Replace your artifact...',
    addSubmissionText: 'Add submission text...',
    updateSubmissionButton: 'Update Submission',
    errors: {
      noMetrics: 'Scenario has no manual metrics',
      alreadyScored: 'Submission has already been scored and can no longer be updated',
      notAltered: 'Submissions have not been altered',
      scoreValue: 'Score must be between 0 and {{maxScore}}',
      downloadFailed: 'Download failed',
      updateFailedWithMessage: 'Metric Score update failed: {{errorMessage}}',
      updateFailed: 'Metric Score update failed',
      addMetricFailedWithMessage: 'Adding a new submission failed: {{errorMessage}}',
      addMetricFailed: 'An error occurred while creating a new submission',
      scoreNotSet: 'Score is not set',
    },
  },
  log: {
    date: 'Timestamp',
    level: 'Level',
    message: 'Message',
    empty: 'No logs available.',
  },
  deployment: {
    empty: 'No deployment available.',
  },
  accountsTable: {
    title: 'Accounts',
    vmName: 'VM Name',
    username: 'Username',
    password: 'Password',
    privatekey: 'Private Key',
    copyButton: 'Copy value',
    copyFail: 'Failed to copy value to clipboard: {{errorMessage}}',
    copySuccess: 'Private key copied to clipboard',
  },
  banners: {
    required: 'Required',
    name: 'Banner name',
    content: 'Banner message content',
    createSuccess: 'Banner created',
    createFail: 'Error creating banner: {{errorMessage}}',
    createFailWithoutMessage: 'Failed to create banner',
    updateSuccess: 'Banner updated',
    updateFail: 'Error updating banner: {{errorMessage}}',
    updateFailWithoutMessage: 'Failed to update banner',
    deleteSuccess: 'Banner deleted',
    deleteFail: 'Error deleting banner: {{errorMessage}}',
    deleteFailWithoutMessage: 'Failed to delete banner',
  },
  fallback: {
    role: 'You do not have any roles assigned to you. Please contact your Ranger administrator.',
  },
  scoreTable: {
    orderPlaceholder: 'Order by',
    scoreDescending: 'Score (highest first)',
    scoreAscending: 'Score (lowest first)',
    nameDescending: 'Name (Z-A)',
    nameAscending: 'Name (A-Z)',
    createdAtDescending: 'Deployment time (newest first)',
    createdAtAscending: 'Deployment time (oldest first)',
    rolePlaceholder: 'Sort by role',
    allRoles: 'All roles',
    errorFetchingRoles: 'Error fetching roles',
  },

};

export default language;


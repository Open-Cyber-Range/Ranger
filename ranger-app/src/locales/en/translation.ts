const language = {
  homepage: 'Welcome to Ranger!',
  appName: 'Ranger',
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
    noDeployments: 'No deployments',
    noDeploymentInfo: 'No deployment info',
    deleteSuccess: 'Exercise {{exerciseName}} deleted',
    deleteFail: 'Failed to delete the exercise {{exerciseName}}',
    easeDevelopment: 'To ease the development of the exercise open: ',
    mustHaveName: 'Exercise must have a name',
    tabs: {
      dashboard: 'Dashboard',
      scores: 'Scores',
      accounts: 'Accounts',
      entities: 'Entity Selector',
      metrics: 'Manual Metrics',
      sdl: 'SDL',
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
      group: {
        placeholder: 'Select group...',
        title: 'Deployment group',
        required: 'Deployment group is required',
      },
      name: {
        title: 'Deployment name',
        required: 'Deployment name is required',
      },
      count: {
        title: 'Deployment count',
        required: 'Deployment count is required',
      },
      startDate: {
        title: 'Deployment start time',
        required: 'Deployment start time is required',
      },
      endDate: {
        title: 'Deployment end time',
        required: 'Deployment end time is required',
        earlierThanStart: 'Deployment end time must be later than start time',
      },
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
  },
  emails: {
    link: 'Emails',
    emailLog: 'Email log:',
    status: 'Status',
    timestamp: 'Timestamp',
    toEntity: 'To Entity',
    from: 'From',
    to: 'To',
    replyTo: 'Reply to',
    subject: 'Subject',
    bcc: 'Bcc',
    cc: 'Cc',
    body: 'Body',
    send: 'Send',
    sendingSuccess: 'Email sent',
    sendingFail: 'Error trying to send the email: {{errorMessage}}',
    sendingFailWithoutMessage: 'Failed to send the email',
    invalidEmailAddress: 'Invalid email address(es): {{invalidEmailAddresses}}',
    noDeployment: 'Cannot find this deployment',
    noUsers: 'No users in the deployment',
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
      body: {
        title: 'Email body',
        required: 'Email body is required',
      },
      emailPlaceholder:
      '...multiple addresses separated by comma or enter',
    },
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
        manualMetrics: 'Manual Metrics',
      },
      events: {
        noEvents: 'Participant is not connected to any Events in this deployment',
        noTriggeredEvents: 'No Events have been triggered yet',
        noDescription: 'Event has no description',
        triggeredAt: 'Event triggered at',
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
      updateFailed: 'Metric Score update failed',
      newManualMetricFailed: 'An error occurred while creating a new submission',
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
  fallback: {
    role: 'You do not have any roles assigned to you. Please contact your Ranger administrator.',
  },
};

export default language;


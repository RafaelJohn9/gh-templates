# Workflow Troubleshooting Guide

## Auto-Version Workflow Not Triggering Other Workflows

### Problem
The auto-version workflow creates and pushes version tags, but other workflows (`build.yml`, `update_changelog.yml`, `update-readme-version.yml`) are not being triggered by these tag pushes.

### Root Cause
GitHub Actions has a security feature where workflows triggered by the default `GITHUB_TOKEN` cannot trigger other workflows. This prevents infinite loops but also prevents legitimate workflow chaining.

### Solution
Configure a Personal Access Token (PAT) to allow the auto-version workflow to trigger other workflows.

#### Step 1: Create a Personal Access Token
1. Go to GitHub Settings → Developer settings → Personal access tokens → Tokens (classic)
2. Click "Generate new token (classic)"
3. Set the following permissions:
   - `contents:write` - Required to push tags
   - `actions:write` - Required to trigger other workflows
   - `metadata:read` - Required for basic repository access
4. Copy the generated token

#### Step 2: Add Token to Repository Secrets
1. Go to your repository → Settings → Secrets and variables → Actions
2. Click "New repository secret"
3. Name: `PAT_TOKEN`
4. Value: Paste the token from Step 1
5. Click "Add secret"

#### Step 3: Verify Configuration
After adding the `PAT_TOKEN` secret:
1. Push a commit to the main branch
2. Check the auto-version workflow logs - you should see "✅ PAT_TOKEN is configured"
3. The workflow should create a new version tag
4. Other workflows should be triggered by the tag push

### Verification
You can verify the fix is working by:
1. Checking that the auto-version workflow shows "PAT_TOKEN is configured" in the logs
2. Confirming that when a tag is pushed, the following workflows are triggered:
   - `build.yml` (Release workflow)
   - `update_changelog.yml` (Update CHANGELOG)
   - `update-readme-version.yml` (Update README Version)

### Fallback Behavior
If `PAT_TOKEN` is not configured, the workflow will:
- Still function and create version tags
- Use the default `GITHUB_TOKEN`
- Show a warning that other workflows won't be triggered
- Provide instructions on how to fix the configuration

### Security Considerations
- The PAT should have minimal required permissions (`contents:write`, `actions:write`, `metadata:read`)
- Store the PAT as a repository secret, never commit it to code
- Consider using a GitHub App token instead of a PAT for organization-wide setups
- Regularly rotate the PAT according to your security policies

## Alternative Solutions

### GitHub App Token
For more advanced setups, you can use a GitHub App token instead of a PAT:
1. Create a GitHub App with the required permissions
2. Install the app on your repository
3. Use the `actions/create-github-app-token` action to generate a token
4. Replace `PAT_TOKEN` usage with the app token

### Manual Workflow Triggers
If you prefer not to use tokens, you can manually trigger dependent workflows:
1. Remove automatic triggers from dependent workflows
2. Add `workflow_dispatch` triggers
3. Use the GitHub API to manually trigger workflows after tag creation

## Related Issues
- [GitHub Actions Documentation: Triggering a workflow from a workflow](https://docs.github.com/en/actions/using-workflows/triggering-a-workflow#triggering-a-workflow-from-a-workflow)
- This addresses the issue described in the repository's issue tracker regarding workflow chaining
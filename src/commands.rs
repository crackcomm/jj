use std::collections::{BTreeSet, HashSet};
use std::ops::Deref;
use std::sync::Mutex;
use clap::{ArgAction, ArgGroup, ArgMatches, CommandFactory, FromArgMatches, Subcommand};
use jujutsu_lib::backend::{CommitId, Timestamp, TreeValue};
use jujutsu_lib::matchers::EverythingMatcher;
use jujutsu_lib::tree::{merge_trees, Tree};
use jujutsu_lib::{conflicts, file_util, git, revset};
    check_stale_working_copy, print_checkout_stats, print_failed_git_export, resolve_base_revs,
    short_commit_description, short_commit_hash, user_error, user_error_with_hint,
    write_commit_summary, Args, CommandError, CommandHelper, RevisionArg, WorkspaceCommandHelper,
use crate::diff_util::{self, DiffFormat, DiffFormatArgs};
use crate::formatter::Formatter;
    /// Instead of resolving one conflict, list all the conflicts
    // TODO: Also have a `--summary` option. `--list` currently acts like
    // `diff --summary`, but should be more verbose.
    #[arg(long, short)]
    list: bool,
    /// Restrict to these paths when searching for a conflict to resolve. We
    /// will attempt to resolve the first conflict we can find. You can use
    /// the `--list` argument to find paths to use here.
    // TODO: Find the conflict we can resolve even if it's not the first one.
    paths: Vec<String>,
        /// The branches to forget.
        #[arg(required_unless_present_any(&["glob"]))]

        /// A glob pattern indicating branches to forget.
        #[arg(action(ArgAction::Append), long = "glob")]
        glob: Vec<String>,
    UpdateStale(WorkspaceUpdateStaleArgs),
/// Update a workspace that has become stale
///
/// For information about stale working copies, see
/// https://github.com/martinvonz/jj/blob/main/docs/working-copy.md.
#[derive(clap::Args, Clone, Debug)]
struct WorkspaceUpdateStaleArgs {}

                format!("'{ui_path}' is not ignored.")
        writeln!(ui, "Rebased {num_rebased} descendant commits")?;
    ui.request_pager();
            ui.request_pager();
            ui.request_pager();
    diff_util::show_diff(
        &from_tree,
        &to_tree,
        matcher.as_ref(),
        &diff_util::diff_formats_for(ui, &args.format),
    diff_util::show_patch(
        &commit,
        &EverythingMatcher,
        &diff_util::diff_formats_for(ui, &args.format),
    ui.request_pager();
        for parent in wc_commit.parents() {
            formatter.write_str("Parent commit: ")?;
            write_commit_summary(
                formatter,
                repo.as_repo_ref(),
                &workspace_id,
                &parent,
                ui.settings(),
            )?;
            formatter.write_str("\n")?;
        }
            formatter.with_label("branch", |formatter| write!(formatter, "{branch_name}"))?;
                write!(formatter, "{branch_name}@{remote_name}")
        let parent_tree = merge_commit_trees(repo.as_repo_ref(), &wc_commit.parents());
            diff_util::show_diff_summary(
    let committer_timestamp = if settings.relative_timestamps() {
        "committer.timestamp().ago()"
        "committer.timestamp()"
            " " label("timestamp", {committer_timestamp})
    let diff_formats = diff_util::diff_formats_for_log(ui, &args.diff_format, args.patch);
                if !diff_formats.is_empty() {
                    diff_util::show_patch(
                        &diff_formats,
                if !diff_formats.is_empty() {
                    diff_util::show_patch(
                        &diff_formats,
    let diff_formats = diff_util::diff_formats_for_log(ui, &args.diff_format, args.patch);
            if !diff_formats.is_empty() {
                    &diff_formats,
            if !diff_formats.is_empty() {
                show_predecessor_patch(formatter, &workspace_command, &commit, &diff_formats)?;
    diff_formats: &[DiffFormat],
    diff_util::show_diff(
        formatter,
        workspace_command,
        &predecessor_tree,
        &commit.tree(),
        &EverythingMatcher,
        diff_formats,
    )
    diff_util::show_diff(
        &from_tree,
        &to.tree(),
        matcher.as_ref(),
        &diff_util::diff_formats_for(ui, &args.format),
    let description_file_path = repo.repo_path().join(format!("description-{random}.txt"));
            .unwrap_or_else(|_| {
                panic!(
                    "failed to open {} for write",
                    description_file_path.display()
                )
            });
        .unwrap_or_else(|_| {
            panic!(
                "failed to open {} for read",
                description_file_path.display()
            )
        });
            "Rebased {num_rebased} descendant commits onto parents of abandoned commits"
    let matcher = workspace_command.matcher_from_values(&args.paths)?;
    let tree = commit.tree();
    let conflicts = tree.conflicts_matching(matcher.as_ref());
    if conflicts.is_empty() {
        return Err(CommandError::CliError(
            "No conflicts found ".to_string()
                + (if args.paths.is_empty() {
                    "at this revision"
                } else {
                    "at the given path(s)"
                }),
        ));
    }
    if args.list {
        let mut formatter = ui.stdout_formatter();
        let formatter = formatter.as_mut();
        for (repo_path, _conflict_id) in conflicts {
            // TODO: Similar to `jj diff --summary`, insert a few letters
            // before the filename to indicate the kind of conflict.
            // E.g. we could have a letter per add : `FF` is a usual conflict
            // between two versions of a file, `FD` is a file vs directory,
            // `FFF` for a merge of three conflicting versions. Additionally,
            // if (# removes) + 1 > (# adds), this indicates the file was deleted
            // in some versions of the conflict. Perhaps that should be `R` for removed.
            writeln!(
                formatter,
                "{}",
                &workspace_command.format_file_path(&repo_path)
            )?;
        }
        return Ok(());
    };

    let (repo_path, _) = conflicts.get(0).unwrap();
    let new_tree_id = workspace_command.run_mergetool(ui, &commit.tree(), repo_path)?;
    from_tree: &Tree,
    to_tree: &Tree,
    let diff_summary_bytes = diff_util::diff_as_bytes(
        workspace_command,
        from_tree,
        to_tree,
        &EverythingMatcher,
        &[DiffFormat::Summary],
    )?;
            &base_tree,
            &middle_tree,
            &middle_tree,
            &commit.tree(),
            writeln!(ui, "Rebased {num_rebased} descendant commits")?;
    writeln!(ui, "Rebased {num_rebased} commits")?;
    writeln!(ui, "Rebased {num_rebased} commits")?;
        let new_child_parents: Vec<Commit> = workspace_command
            .try_collect()?;
            &new_child_parents,
            "Also rebased {num_rebased_descendants} descendant commits onto parent of rebased \
             commit"
                return Err(user_error(format!("No such branch: {branch_name}")));
    fn find_globs(view: &View, globs: &[String]) -> Result<Vec<String>, CommandError> {
        let globs: Vec<glob::Pattern> = globs
            .iter()
            .map(|glob| glob::Pattern::new(glob))
            .try_collect()?;
        let matching_branches = view
            .branches()
            .iter()
            .map(|(branch_name, _branch_target)| branch_name)
            .filter(|branch_name| globs.iter().any(|glob| glob.matches(branch_name)))
            .cloned()
            .collect();
        Ok(matching_branches)
    }

                        format!("Branch already exists: {branch_name}"),
        BranchSubcommand::Forget { names, glob } => {
            let globbed_names = find_globs(view, glob)?;
            let names: BTreeSet<String> = names.iter().cloned().chain(globbed_names).collect();
            let branch_term = make_branch_term(names.iter().collect_vec().as_slice());
            let mut tx = workspace_command.start_transaction(&format!("forget {branch_term}"));
                tx.mut_repo().remove_branch(&branch_name);
        formatter.with_label("branch", |formatter| write!(formatter, "{name}"))?;
            formatter.with_label("branch", |formatter| write!(formatter, "@{remote}"))?;
                    write!(formatter, " (ahead by {remote_ahead_count} commits)")?;
                    write!(formatter, " (behind by {local_ahead_count} commits)")?;
                        " (ahead by {remote_ahead_count} commits, behind by {local_ahead_count} \
                         commits)"
            writeln!(ui, "{parse:?}")?;
                writeln!(ui, "  Level {i}:")?;
                    formatter.write_str(&format!("\n{key}: {value}"))
        WorkspaceCommands::UpdateStale(command_matches) => {
            cmd_workspace_update_stale(ui, command, command_matches)
        }
            "Workspace named '{name}' already exists"
fn cmd_workspace_update_stale(
    ui: &mut Ui,
    command: &CommandHelper,
    _args: &WorkspaceUpdateStaleArgs,
) -> Result<(), CommandError> {
    let workspace = command.load_workspace(ui)?;
    let mut workspace_command = command.resolve_operation(ui, workspace)?;
    let repo = workspace_command.repo().clone();
    let workspace_id = workspace_command.workspace_id();
    let (mut locked_wc, desired_wc_commit) =
        workspace_command.unsafe_start_working_copy_mutation()?;
    match check_stale_working_copy(&locked_wc, &desired_wc_commit, repo.clone()) {
        Ok(_) => {
            locked_wc.discard();
            ui.write("Nothing to do (the working copy is not stale).\n")?;
        }
        Err(_) => {
            // TODO: First commit the working copy
            let stats = locked_wc
                .check_out(&desired_wc_commit.tree())
                .map_err(|err| {
                    CommandError::InternalError(format!(
                        "Failed to check out commit {}: {}",
                        desired_wc_commit.id().hex(),
                        err
                    ))
                })?;
            locked_wc.finish(repo.op_id().clone());
            ui.write("Working copy now at: ")?;
            write_commit_summary(
                ui.stdout_formatter().as_mut(),
                repo.as_repo_ref(),
                &workspace_id,
                &desired_wc_commit,
                ui.settings(),
            )?;
            ui.write("\n")?;
            print_checkout_stats(ui, stats)?;
        }
    }
    Ok(())
}

            writeln!(ui, "{ui_path}")?;
        let paths_to_add: Vec<_> = args
            .try_collect()?;
        let paths_to_remove: Vec<_> = args
            .try_collect()?;
fn absolute_git_source(cwd: &Path, source: &str) -> String {
    // Git appears to turn URL-like source to absolute path if local git directory
    // exits, and fails because '$PWD/https' is unsupported protocol. Since it would
    // be tedious to copy the exact git (or libgit2) behavior, we simply assume a
    // source containing ':' is a URL, SSH remote, or absolute path with Windows
    // drive letter.
    if !source.contains(':') && Path::new(source).exists() {
        // It's less likely that cwd isn't utf-8, so just fall back to original source.
        cwd.join(source)
            .into_os_string()
            .into_string()
            .unwrap_or_else(|_| source.to_owned())
    } else {
        source.to_owned()
    }
}

    let source = absolute_git_source(ui.cwd(), &args.source);
        .or_else(|| clone_destination_for_source(&source))
    let clone_result = do_git_clone(ui, command, &source, &wc_path);
    ui.prompt(&format!("Username for {url}")).ok()
    ui.prompt_password(&format!("Passphrase for {url}: ")).ok()
        let qualified_name = format!("refs/heads/{branch_name}");
        .ok_or_else(|| user_error(format!("Branch {branch_name} doesn't exist")))?;
            Err(user_error(format!("Branch {branch_name} is conflicted")))
            "Branch {branch_name}@{remote_name} is conflicted"
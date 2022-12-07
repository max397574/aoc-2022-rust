local curdir = ""
local files = {}
local dirs = {}
local input = io.open("input/2022/day7.txt")
local lines = input and input:lines()
local line_table = {}
for line in lines do
    table.insert(line_table, line)
end
for i = 1, #line_table do
    local line = line_table[i]
    if line:sub(1, 1) == "$" and line:sub(3, 4) == "cd" then
        local arg = line:sub(6, -1)
        if arg == "/" then
            curdir = ""
        elseif arg == ".." then
            local curdir_table = vim.split(curdir, "/")
            table.remove(curdir_table)
            if #curdir_table == 1 and curdir_table[1] == "" then
                curdir_table = { "", "" }
            end
            curdir = table.concat(curdir_table, "/")
        else
            if curdir == "/" then
                curdir = ""
            end
            curdir = curdir .. "/" .. arg
        end
        if not vim.tbl_contains(dirs, curdir) then
            table.insert(dirs, curdir)
        end
    elseif line:sub(1, 1) == "$" and line:sub(3, 4) == "ls" then
        i = i + 1
        while i <= #line_table and line_table[i]:sub(1, 1) ~= "$" do
            if line_table[i]:sub(1, 1) ~= "d" then
                local size, file_name = unpack(vim.split(line_table[i], " "))
                files[curdir .. "/" .. file_name] = size
            end
            i = i + 1
        end
    end
end

local directory_sizes = {}

for _, directory in ipairs(dirs) do
    for file, size in pairs(files) do
        if file:sub(1, #directory) == directory then
            if not vim.tbl_contains(vim.tbl_keys(directory_sizes), directory) then
                directory_sizes[directory] = 0
            end
            directory_sizes[directory] = directory_sizes[directory] + size
        end
    end
end
local sum = 0
for _, size in pairs(directory_sizes) do
    if size <= 100000 then
        sum = sum + size
    end
end
print("part1", sum)
local unused_space = 70000000 - directory_sizes["/"]
local smallest = 100000000
for _, size in pairs(directory_sizes) do
    if size + unused_space >= 30000000 and size < smallest then
        smallest = size
    end
end
print("part2", smallest)

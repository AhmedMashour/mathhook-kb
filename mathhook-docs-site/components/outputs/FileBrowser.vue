<template>
  <div class="file-browser">
    <!-- Header with bulk actions -->
    <div class="flex items-center justify-between mb-4 pb-3 border-b border-logic-navy-700/50">
      <div class="flex items-center gap-3">
        <label class="flex items-center gap-2 cursor-pointer text-sm text-chalk-400 hover:text-chalk">
          <input
            type="checkbox"
            :checked="allSelected"
            :indeterminate="someSelected && !allSelected"
            @change="toggleSelectAll"
            class="w-4 h-4 rounded border-logic-navy-600 bg-logic-navy-800 text-rust-core focus:ring-rust-core/50"
          />
          <span>{{ selectedCount > 0 ? `${selectedCount} selected` : 'Select all' }}</span>
        </label>
      </div>

      <div class="flex items-center gap-2">
        <button
          v-if="selectedCount > 0"
          @click="downloadSelected"
          class="flex items-center gap-2 px-3 py-1.5 rounded-lg text-sm font-medium transition-all duration-200"
          :class="downloadButtonClass"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
          </svg>
          Download Selected ({{ selectedCount }})
        </button>
      </div>
    </div>

    <!-- Categories/Folders -->
    <div class="space-y-4">
      <div v-for="(categoryFiles, category) in groupedFiles" :key="category" class="category-group">
        <!-- Category Header -->
        <div class="flex items-center gap-2 py-2 px-3 rounded-lg hover:bg-logic-navy-700/30 transition-colors group">
          <!-- Section Select All Checkbox -->
          <input
            type="checkbox"
            :checked="isCategoryFullySelected(category)"
            :indeterminate="isCategoryPartiallySelected(category)"
            @change.stop="toggleCategorySelection(category)"
            @click.stop
            class="w-4 h-4 rounded border-logic-navy-600 bg-logic-navy-800 focus:ring-rust-core/50 cursor-pointer"
            :class="checkboxColorClass"
            :title="`Select all files in ${formatCategoryName(category)}`"
          />

          <button
            @click="toggleCategory(category)"
            class="flex-1 flex items-center gap-2"
          >
            <svg
              class="w-4 h-4 text-chalk-400 transition-transform duration-200"
              :class="{ 'rotate-90': expandedCategories.includes(category) }"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
            </svg>
            <svg class="w-5 h-5" :class="folderColorClass" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
            </svg>
            <span class="text-chalk font-medium capitalize">{{ formatCategoryName(category) }}</span>
            <span class="text-chalk-500 text-sm">({{ categoryFiles.length }} files)</span>
            <span
              v-if="getCategorySelectedCount(category) > 0"
              class="text-xs px-2 py-0.5 rounded-full"
              :class="badgeColorClass"
            >
              {{ getCategorySelectedCount(category) }} selected
            </span>
          </button>
        </div>

        <!-- Files in Category -->
        <Transition name="expand">
          <div v-show="expandedCategories.includes(category)" class="ml-6 mt-2 space-y-1">
            <div
              v-for="file in categoryFiles"
              :key="file.path"
              class="flex items-center gap-3 py-2 px-3 rounded-lg hover:bg-logic-navy-700/20 transition-colors group"
            >
              <!-- Checkbox -->
              <input
                type="checkbox"
                :checked="selectedFiles.includes(file.path)"
                @change="toggleFile(file.path)"
                class="w-4 h-4 rounded border-logic-navy-600 bg-logic-navy-800 focus:ring-rust-core/50"
                :class="checkboxColorClass"
              />

              <!-- File Icon -->
              <component :is="getFileIcon(file.extension)" class="w-5 h-5" :class="iconColorClass" />

              <!-- File Info -->
              <div class="flex-1 min-w-0">
                <div class="text-chalk text-sm font-medium truncate">{{ file.name }}</div>
                <div class="text-chalk-500 text-xs">{{ file.title || formatFileName(file.name) }}</div>
              </div>

              <!-- File Size -->
              <span class="text-chalk-500 text-xs hidden sm:block">{{ formatFileSize(file.size) }}</span>

              <!-- Actions -->
              <div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                <!-- Open in Colab (for colab files) -->
                <a
                  v-if="file.colabUrl"
                  :href="file.colabUrl"
                  target="_blank"
                  class="p-1.5 rounded-lg hover:bg-logic-navy-600/50 transition-colors"
                  :class="actionColorClass"
                  title="Open in Google Colab"
                >
                  <svg class="w-4 h-4" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
                  </svg>
                </a>

                <!-- View/Open -->
                <a
                  :href="file.url"
                  target="_blank"
                  class="p-1.5 rounded-lg hover:bg-logic-navy-600/50 transition-colors"
                  :class="actionColorClass"
                  title="View file"
                >
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                  </svg>
                </a>

                <!-- Download -->
                <button
                  @click="downloadFile(file)"
                  class="p-1.5 rounded-lg hover:bg-logic-navy-600/50 transition-colors"
                  :class="actionColorClass"
                  title="Download file"
                >
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                  </svg>
                </button>
              </div>
            </div>
          </div>
        </Transition>
      </div>
    </div>

    <!-- Empty State -->
    <div v-if="files.length === 0" class="text-center py-8">
      <svg class="w-12 h-12 mx-auto text-chalk-600 mb-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
      </svg>
      <p class="text-chalk-500">No files available</p>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'

const props = defineProps({
  files: {
    type: Array,
    default: () => []
  },
  color: {
    type: String,
    default: 'rust',
    validator: (v) => ['rust', 'amber', 'green', 'violet', 'cyan'].includes(v)
  }
})

const emit = defineEmits(['download', 'downloadMultiple'])

// State
const selectedFiles = ref([])
const expandedCategories = ref([])

// Initialize with first category expanded
onMounted(() => {
  const categories = Object.keys(groupedFiles.value)
  if (categories.length > 0) {
    expandedCategories.value = [categories[0]]
  }
})

// Computed
const groupedFiles = computed(() => {
  const groups = {}
  props.files.forEach(file => {
    const category = file.category || 'uncategorized'
    if (!groups[category]) {
      groups[category] = []
    }
    groups[category].push(file)
  })
  // Sort categories alphabetically
  return Object.keys(groups)
    .sort()
    .reduce((sorted, key) => {
      sorted[key] = groups[key]
      return sorted
    }, {})
})

const selectedCount = computed(() => selectedFiles.value.length)
const allSelected = computed(() =>
  props.files.length > 0 && selectedFiles.value.length === props.files.length
)
const someSelected = computed(() => selectedFiles.value.length > 0)

// Color classes
const colorClasses = {
  rust: {
    folder: 'text-rust-core',
    icon: 'text-rust-core',
    checkbox: 'text-rust-core',
    action: 'text-rust-core hover:text-rust-core-300',
    button: 'bg-rust-core/10 text-rust-core hover:bg-rust-core/20 border border-rust-core/30'
  },
  amber: {
    folder: 'text-amber-500',
    icon: 'text-amber-500',
    checkbox: 'text-amber-500',
    action: 'text-amber-500 hover:text-amber-400',
    button: 'bg-amber-500/10 text-amber-500 hover:bg-amber-500/20 border border-amber-500/30'
  },
  green: {
    folder: 'text-step-green',
    icon: 'text-step-green',
    checkbox: 'text-step-green',
    action: 'text-step-green hover:text-step-green-300',
    button: 'bg-step-green/10 text-step-green hover:bg-step-green/20 border border-step-green/30'
  },
  violet: {
    folder: 'text-violet-500',
    icon: 'text-violet-500',
    checkbox: 'text-violet-500',
    action: 'text-violet-500 hover:text-violet-400',
    button: 'bg-violet-500/10 text-violet-500 hover:bg-violet-500/20 border border-violet-500/30'
  },
  cyan: {
    folder: 'text-solve-cyan',
    icon: 'text-solve-cyan',
    checkbox: 'text-solve-cyan',
    action: 'text-solve-cyan hover:text-solve-cyan-300',
    button: 'bg-solve-cyan/10 text-solve-cyan hover:bg-solve-cyan/20 border border-solve-cyan/30'
  }
}

const folderColorClass = computed(() => colorClasses[props.color].folder)
const iconColorClass = computed(() => colorClasses[props.color].icon)
const checkboxColorClass = computed(() => colorClasses[props.color].checkbox)
const actionColorClass = computed(() => colorClasses[props.color].action)
const downloadButtonClass = computed(() => colorClasses[props.color].button)
const badgeColorClass = computed(() => colorClasses[props.color].button)

// Category selection helpers
function isCategoryFullySelected(category) {
  const categoryFiles = groupedFiles.value[category] || []
  if (categoryFiles.length === 0) return false
  return categoryFiles.every(f => selectedFiles.value.includes(f.path))
}

function isCategoryPartiallySelected(category) {
  const categoryFiles = groupedFiles.value[category] || []
  const selectedInCategory = categoryFiles.filter(f => selectedFiles.value.includes(f.path))
  return selectedInCategory.length > 0 && selectedInCategory.length < categoryFiles.length
}

function getCategorySelectedCount(category) {
  const categoryFiles = groupedFiles.value[category] || []
  return categoryFiles.filter(f => selectedFiles.value.includes(f.path)).length
}

function toggleCategorySelection(category) {
  const categoryFiles = groupedFiles.value[category] || []
  const isFullySelected = isCategoryFullySelected(category)

  if (isFullySelected) {
    // Deselect all files in this category
    categoryFiles.forEach(f => {
      const index = selectedFiles.value.indexOf(f.path)
      if (index !== -1) {
        selectedFiles.value.splice(index, 1)
      }
    })
  } else {
    // Select all files in this category
    categoryFiles.forEach(f => {
      if (!selectedFiles.value.includes(f.path)) {
        selectedFiles.value.push(f.path)
      }
    })
  }
}

// Methods
function toggleCategory(category) {
  const index = expandedCategories.value.indexOf(category)
  if (index === -1) {
    expandedCategories.value.push(category)
  } else {
    expandedCategories.value.splice(index, 1)
  }
}

function toggleFile(path) {
  const index = selectedFiles.value.indexOf(path)
  if (index === -1) {
    selectedFiles.value.push(path)
  } else {
    selectedFiles.value.splice(index, 1)
  }
}

function toggleSelectAll() {
  if (allSelected.value) {
    selectedFiles.value = []
  } else {
    selectedFiles.value = props.files.map(f => f.path)
  }
}

function downloadFile(file) {
  // Create download link
  const link = document.createElement('a')
  link.href = file.url
  link.download = file.name
  link.target = '_blank'
  document.body.appendChild(link)
  link.click()
  document.body.removeChild(link)
  emit('download', file)
}

function downloadSelected() {
  const filesToDownload = props.files.filter(f => selectedFiles.value.includes(f.path))

  // Download each file with a small delay to prevent browser blocking
  filesToDownload.forEach((file, index) => {
    setTimeout(() => {
      downloadFile(file)
    }, index * 300)
  })

  emit('downloadMultiple', filesToDownload)
}

function formatCategoryName(category) {
  return category
    .split('-')
    .map(word => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ')
}

function formatFileName(name) {
  // Remove extension and format
  return name
    .replace(/\.[^/.]+$/, '')
    .split('-')
    .map(word => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ')
}

function formatFileSize(bytes) {
  if (!bytes) return ''
  const units = ['B', 'KB', 'MB', 'GB']
  let size = bytes
  let unitIndex = 0
  while (size >= 1024 && unitIndex < units.length - 1) {
    size /= 1024
    unitIndex++
  }
  return `${size.toFixed(1)} ${units[unitIndex]}`
}

function getFileIcon(extension) {
  // Return appropriate icon based on file extension
  const iconMap = {
    ipynb: 'OutputsIconsNotebookIcon',
    md: 'OutputsIconsDocumentIcon',
    tex: 'OutputsIconsDocumentIcon',
    pdf: 'OutputsIconsDocumentIcon',
    json: 'OutputsIconsTerminalIcon',
    vue: 'OutputsIconsComputerIcon'
  }
  return iconMap[extension] || 'OutputsIconsDocumentIcon'
}
</script>

<style scoped>
.expand-enter-active,
.expand-leave-active {
  transition: all 0.3s ease;
  overflow: hidden;
}

.expand-enter-from,
.expand-leave-to {
  opacity: 0;
  max-height: 0;
}

.expand-enter-to,
.expand-leave-from {
  opacity: 1;
  max-height: 1000px;
}

/* Custom checkbox styling */
input[type="checkbox"] {
  cursor: pointer;
}

input[type="checkbox"]:indeterminate {
  background-image: url("data:image/svg+xml,%3csvg viewBox='0 0 16 16' fill='white' xmlns='http://www.w3.org/2000/svg'%3e%3crect width='8' height='2' x='4' y='7' rx='1'/%3e%3c/svg%3e");
}
</style>
